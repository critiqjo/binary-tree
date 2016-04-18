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

pub trait BinaryTree {
    type Node: Node;

    fn root(&self) -> &Self::Node;
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
    fn walk_mut<FF, FB>(&mut self, mut forth: FF, mut back: FB)
        where FF: FnMut(&mut Self) -> WalkAction,
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
        if let Some((mut sst, action)) = stack.pop() {
            back(&mut sst, action); // the final action is irrelevant
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
        }
        back(self, root_action);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum WalkAction {
    Left,
    Right,
    Stop,
}
