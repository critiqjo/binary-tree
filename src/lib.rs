//! Provides a collection of binary tree based data structures and algorithms.
//!
//! ## Terminology
//!
//! * The root of a tree is considered to be at the top.
//! * Height of a node is the length of the longest path to _its_ leaves. Thus
//!   all leaf nodes have zero height.

#[cfg(feature="quickcheck")]
extern crate quickcheck;

pub mod cow;
pub mod count;
pub mod iter;
pub mod test;
pub mod unbox;

use std::mem;
use std::ops::DerefMut;

pub trait BinaryTree {
    type Node: Node;

    fn root(&self) -> Option<&Self::Node>;
}

unsafe fn borrow<'a, T, U>(raw: *const T, _: &'a U) -> &'a T {
    &*raw
}

unsafe fn borrow_mut<'a, T, U>(raw: *mut T, _: &'a U) -> &'a mut T {
    &mut *raw
}

/// Generic methods for traversing a binary tree.
pub trait Node {
    type Value;

    /// Get a reference to the left subtree
    fn left(&self) -> Option<&Self>;

    /// Get a reference to the right subtree
    fn right(&self) -> Option<&Self>;

    /// Returns the value of the current node.
    fn value(&self) -> &Self::Value;

    /// Walk down the tree
    fn walk<'a, F>(&'a self, mut step_in: F)
        where F: FnMut(&'a Self) -> WalkAction
    {
        use WalkAction::*;

        let mut subtree = Some(self);
        while let Some(mut st) = subtree {
            let action = step_in(&mut st);
            subtree = match action {
                Left => st.left(),
                Right => st.right(),
                Stop => break,
            };
        }
    }
}

/// Mutating methods on a Binary Tree node.
pub trait NodeMut: Node + Sized {
    type NodePtr: Sized + DerefMut<Target = Self>;

    /// Try to detach the left sub-tree
    fn detach_left(&mut self) -> Option<Self::NodePtr>;

    /// Try to detach the right sub-tree
    fn detach_right(&mut self) -> Option<Self::NodePtr>;

    /// Replace the left subtree with `tree` and return the old one.
    fn insert_left(&mut self, tree: Option<Self::NodePtr>) -> Option<Self::NodePtr>;

    /// Replace the right subtree with `tree` and return the old one.
    fn insert_right(&mut self, tree: Option<Self::NodePtr>) -> Option<Self::NodePtr>;

    /// Returns a mutable reference to the value of the current node.
    fn value_mut(&mut self) -> &mut Self::Value;

    /// Consume a Node and return its parts: (value, left, right)
    fn into_parts(self) -> (Self::Value, Option<Self::NodePtr>, Option<Self::NodePtr>);

    /// Returns a mutable reference to the left child
    fn left_mut(&mut self) -> Option<&mut Self>;

    /// Returns a mutable reference to the right child
    fn right_mut(&mut self) -> Option<&mut Self>;

    /// Try to rotate the tree left if right subtree exists
    fn rotate_left(&mut self) -> Result<(), ()> {
        if let Some(mut self2) = self.detach_right() {
            let mid = self2.detach_left();
            self.insert_right(mid);
            mem::swap(self, &mut self2);
            self.insert_left(Some(self2));
            Ok(())
        } else {
            Err(())
        }
    }

    /// Try to rotate the tree right if left subtree exists
    fn rotate_right(&mut self) -> Result<(), ()> {
        if let Some(mut self2) = self.detach_left() {
            let mid = self2.detach_right();
            self.insert_left(mid);
            mem::swap(self, &mut self2);
            self.insert_right(Some(self2));
            Ok(())
        } else {
            Err(())
        }
    }

    /// Simple mutable walk
    ///
    /// Note that the type of `step_in` is almost identical to that in
    /// `Node::walk`, but not exactly so. Here, `step_in` does not get a
    /// reference which lives as long as `self` so that it cannot leak
    /// references out to its environment.
    fn walk_mut<'a, FI, FS>(&'a mut self, mut step_in: FI, stop: FS)
        where FI: FnMut(&Self) -> WalkAction,
              FS: FnOnce(&'a mut Self)
    {
        use WalkAction::*;

        let mut node: *mut _ = self;
        loop {
            let action = {
                let pin = ();
                step_in(unsafe { borrow(node, &pin) })
            };
            let next = match action {
                Left => unsafe { borrow_mut(node, self) }.left_mut(),
                Right => unsafe { borrow_mut(node, self) }.right_mut(),
                Stop => break,
            };
            if let Some(st) = next {
                node = st;
            } else {
                break;
            }
        }
        stop(unsafe { borrow_mut(node, self) });
    }

    /// Walks down the tree by detaching subtrees, then up reattaching them
    /// back. `step_in` should guide the path taken, `stop` will be called on
    /// the node where either `step_in` returned `Stop` or it was not possible
    /// to proceed. Then `step_out` will be called for each node along the way
    /// to root, except the final one (that for which `stop` was called).
    fn walk_reshape<FI, FS, FO>(&mut self, mut step_in: FI, stop: FS, mut step_out: FO)
        where FI: FnMut(&mut Self) -> WalkAction,
              FS: FnOnce(&mut Self),
              FO: FnMut(&mut Self, WalkAction)
    {
        use WalkAction::*;

        let mut stack = Vec::with_capacity(8);
        let root_action = step_in(self);
        let mut subtree = match root_action {
            Left => self.detach_left(),
            Right => self.detach_right(),
            Stop => None,
        };
        let mut action = root_action;
        while action != Stop {
            if let Some(mut st) = subtree {
                action = step_in(&mut st);
                subtree = match action {
                    Left => st.detach_left(),
                    Right => st.detach_right(),
                    Stop => None,
                };
                stack.push((st, action));
            } else {
                break;
            }
        }
        if let Some((mut sst, _)) = stack.pop() {
            //               -^- the final action is irrelevant
            stop(&mut sst);
            while let Some((mut st, action)) = stack.pop() {
                match action {
                    Left => st.insert_left(Some(sst)),
                    Right => st.insert_right(Some(sst)),
                    Stop => unreachable!(),
                };
                step_out(&mut st, action);
                sst = st;
            }
            match root_action {
                Left => self.insert_left(Some(sst)),
                Right => self.insert_right(Some(sst)),
                Stop => unreachable!(),
            };
            step_out(self, root_action);
        } else {
            stop(self)
        }
    }

    /// Insert `new_node` in-order before `self`. `step_out` will be invoked for
    /// all nodes in path from (excluding) the point of insertion, to
    /// (including) `self`, unless `self` is the point of insertion.
    fn insert_before<F>(&mut self, new_node: Self::NodePtr, mut step_out: F)
        where F: FnMut(&mut Self, WalkAction)
    {
        use WalkAction::*;

        if let Some(mut left) = self.detach_left() {
            left.walk_reshape(|_| Right,
                              move |node| {
                                  node.insert_right(Some(new_node));
                              },
                              &mut step_out);
            self.insert_left(Some(left));
            step_out(self, Left);
        } else {
            self.insert_left(Some(new_node));
        }
    }

    /// Extract out a node. This can be used in conjuction with `try_remove` to
    /// remove any node except the root.
    ///
    /// The closure `extract` should try to take out the desired node from its
    /// first argument and move it to its second argument (possibly using
    /// `try_remove`). If it was unable to do it, the final node that was
    /// visited will be taken out and returned, unless it is the root itself.
    ///
    /// Note that, similar to `walk_reshape`, `step_out` is not called for the
    /// finally visited node (that for which `extract` was called).
    ///
    /// See the source of `CountTree::remove` for an example use.
    fn walk_extract<FI, FE, FO>(&mut self, step_in: FI, extract: FE, mut step_out: FO) -> Option<Self::NodePtr>
        where FI: FnMut(&mut Self) -> WalkAction,
              FE: FnOnce(&mut Self, &mut Option<Self::NodePtr>),
              FO: FnMut(&mut Self, WalkAction)
    {
        use WalkAction::*;

        let ret = std::cell::RefCell::new(None);
        self.walk_reshape(step_in,
                          |node| extract(node, &mut *ret.borrow_mut()),
                          |node, action| {
                              if ret.borrow().is_none() {
                                  // take out the last visited node if `extract` was unable to
                                  *ret.borrow_mut() = match action {
                                      Left => node.detach_left(),
                                      Right => node.detach_right(),
                                      Stop => unreachable!(),
                                  };
                              }
                              step_out(node, action);
                          });
        ret.into_inner()
    }

    /// Replace this node with one of its descendant, returns `None` if it has
    /// no children.
    fn try_remove<F>(&mut self, mut step_out: F) -> Option<Self::NodePtr>
        where F: FnMut(&mut Self, WalkAction)
    {
        use WalkAction::*;

        match (self.detach_left(), self.detach_right()) {
            (Some(mut left), right @ Some(_)) => {
                // fetch the rightmost descendant of left into pio (previous-in-order of self)
                let mut pio = left.walk_extract(|_| Right,
                                                |node, ret| {
                                                    if let Some(mut left) = node.detach_left() {
                                                        // the rightmost node has a left child
                                                        mem::swap(&mut *left, node);
                                                        *ret = Some(left);
                                                    }
                                                },
                                                &mut step_out);
                if let Some(ref mut pio) = pio {
                    pio.insert_left(Some(left));
                } else {
                    // left had no right child
                    pio = Some(left);
                }
                let mut pio = pio.unwrap();
                pio.insert_right(right);
                step_out(&mut pio, Left);
                mem::swap(&mut *pio, self);
                Some(pio) // old self
            }
            (Some(mut left), None) => {
                mem::swap(&mut *left, self);
                Some(left)
            }
            (None, Some(mut right)) => {
                mem::swap(&mut *right, self);
                Some(right)
            }
            (None, None) => {
                None
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
/// List of actions during a `Node::walk` or `NodeMut::walk_*`.
pub enum WalkAction {
    /// Enter(ed) the left child
    Left,
    /// Enter(ed) the right child
    Right,
    /// Stop walking
    Stop,
}
