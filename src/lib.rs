//! Provides a collection of binary tree based data structures and algorithms.
//!
//! ## Terminology
//!
//! * The root of a tree is considered to be at the _top_.
//! * _Height_ of a node is the length of the longest path to _its_ leaves.
//!   Height of a leaf node is defined to be zero.

use std::mem;
use std::ops::DerefMut;

pub mod cow;
pub mod count;
pub mod iter;
pub mod test;
pub mod unbox;

pub trait BinaryTree {
    type Node: Node;

    fn root(&self) -> Option<&Self::Node>;
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

    /// Consume a Node and return its value
    fn value_owned(self) -> Self::Value;

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

    /// Walks down the tree by detaching subtrees, then up reattaching them
    /// back. `step_in` should guide the path taken, `stop` will be called on
    /// the node where either `step_in` returned `Stop` or it was not possible
    /// to proceed. Then `step_out` will be called for each node, except the
    /// final one, along the way.
    fn walk_mut<FI, FS, FO>(&mut self, mut step_in: FI, stop: FS, mut step_out: FO)
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
            left.walk_mut(|_| Right,
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

    /// Replace this node with one of its descendant, returns `None` if it has no children.
    fn try_remove<F>(&mut self, mut step_out: F) -> Option<Self::NodePtr>
        where F: FnMut(&mut Self, WalkAction)
    {
        use std::cell::RefCell;
        use WalkAction::*;

        if let Some(mut left) = self.detach_left() {
            if self.right().is_none() {
                mem::swap(&mut *left, self);
                Some(left)
            } else {
                let ret: RefCell<Option<_>> = RefCell::new(None);
                // fetch the rightmost descendant of left into ret
                left.walk_mut(|_| Right,
                              |node| {
                                  if let Some(mut left) = node.detach_left() {
                                      // replace the rightmost node with its left child
                                      mem::swap(&mut *left, node);
                                      *ret.borrow_mut() = Some(left);
                                  }
                              },
                              |node, action| {
                                  if ret.borrow().is_none() {
                                      *ret.borrow_mut() = match action {
                                          Left => node.detach_left(),
                                          Right => node.detach_right(),
                                          Stop => unreachable!(),
                                      };
                                  }
                                  step_out(node, action);
                              });
                let mut ret = ret.into_inner();
                if let Some(ref mut ret) = ret {
                    ret.insert_left(Some(left));
                } else {
                    // left had no right child
                    ret = Some(left);
                }
                let mut ret = ret.unwrap();
                ret.insert_right(self.detach_right());
                mem::swap(&mut *ret, self);
                step_out(self, Left);
                Some(ret)
            }
        } else if let Some(mut right) = self.detach_right() {
            mem::swap(&mut *right, self);
            Some(right)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum WalkAction {
    Left,
    Right,
    Stop,
}
