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
pub mod plain;
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
    fn walk<'a, F>(&'a self, mut forth: F)
        where F: FnMut(&'a Self) -> WalkAction,
    {
        use WalkAction::*;

        let mut subtree = Some(self);
        while let Some(mut st) = subtree {
            let action = forth(&mut st);
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
    type NodePtr: Sized + DerefMut<Target=Self>;

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

    /// Insert `new_node` in-order before `self`. `back` will be invoked for all
    /// nodes in path from (excluding) the point of insertion, to (including)
    /// `self`, unless `self` is the point of insertion.
    fn insert_before<FB>(&mut self, new_node: Self::NodePtr, mut back: FB)
        where FB: FnMut(&mut Self, WalkAction)
    {
        use WalkAction::*;

        if let Some(mut left) = self.detach_left() {
            left.walk_mut(|_| Right,
                          move |node| {
                              node.insert_right(Some(new_node));
                          },
                          &mut back);
            self.insert_left(Some(left));
            back(self, Left);
        } else {
            self.insert_left(Some(new_node));
        }
    }

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

    /// Walks down the tree by detaching subtrees, then up reattaching them back.
    fn walk_mut<FF, FS, FB>(&mut self, mut forth: FF, stop: FS, mut back: FB)
        where FF: FnMut(&mut Self) -> WalkAction,
              FS: FnOnce(&mut Self),
              FB: FnMut(&mut Self, WalkAction),
    {
        use WalkAction::*;

        let mut stack = Vec::with_capacity(8);
        let root_action = forth(self);
        let mut subtree = match root_action {
            Left => self.detach_left(),
            Right => self.detach_right(),
            Stop => None,
        };
        let mut action = root_action;
        while action != Stop {
            if let Some(mut st) = subtree {
                action = forth(&mut st);
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
        if let Some((mut sst, _)) = stack.pop() { // the final action is irrelevant
            stop(&mut sst);
            while let Some((mut st, action)) = stack.pop() {
                match action {
                    Left => st.insert_left(Some(sst)),
                    Right => st.insert_right(Some(sst)),
                    Stop => unreachable!(),
                };
                back(&mut st, action);
                sst = st;
            }
            match root_action {
                Left => self.insert_left(Some(sst)),
                Right => self.insert_right(Some(sst)),
                Stop => unreachable!(),
            };
            back(self, root_action);
        } else {
            stop(self)
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum WalkAction {
    Left,
    Right,
    Stop,
}
