//! Provides a collection of binary tree based data structures and algorithms.
//!
//! ## Terminology
//!
//! * The root of a tree is considered to be at the _top_.
//! * _Height_ of a node is the length of the longest path to _its_ leaves.
//!   Height of a leaf node is defined to be zero.
//! * In this crate, a tree and a node is handled uniformly.

#![feature(box_syntax)]

use std::mem;
use std::ops::DerefMut;

pub mod cow;
pub mod count;
pub mod iter;
pub mod plain;

/// Generic methods on binary trees. Calling any of these methods directly on a
/// self-balancing binary tree may make it imbalanced.
pub trait BinaryTree: Sized {
    type Value;
    type Subtree: Sized + DerefMut<Target=Self>;

    /// Get a reference to the left subtree
    fn left(&self) -> Option<&Self::Subtree>;

    /// Get a reference to the right subtree
    fn right(&self) -> Option<&Self::Subtree>;

    /// Try to detach the left sub-tree
    fn detach_left(&mut self) -> Option<Self::Subtree>;

    /// Try to detach the right sub-tree
    fn detach_right(&mut self) -> Option<Self::Subtree>;

    /// Replace the left subtree with `tree` and return the old one.
    fn insert_left(&mut self, tree: Option<Self::Subtree>) -> Option<Self::Subtree>;

    /// Replace the right subtree with `tree` and return the old one.
    fn insert_right(&mut self, tree: Option<Self::Subtree>) -> Option<Self::Subtree>;

    /// Returns the value of the current node.
    fn value(&self) -> &Self::Value;

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
}

#[derive(Clone, Copy, PartialEq)]
pub enum WalkAction {
    Left,
    Right,
    Stop,
}

/// Walks down the tree by detaching subtrees, then reattaches them back.
pub fn walk_mut<T, F>(root: &mut T, mut f: F)
    where T: BinaryTree,
          F: FnMut(&mut T) -> WalkAction
{
    use WalkAction::*;

    let mut stack = Vec::with_capacity(8);
    let root_action = f(root);
    let mut subtree = match root_action {
        Left => root.detach_left(),
        Right => root.detach_right(),
        Stop => None,
    };
    let mut action = root_action;
    while action != Stop {
        if let Some(mut st) = subtree {
            action = f(&mut st);
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
        while let Some((mut st, action)) = stack.pop() {
            match action {
                Left => st.insert_left(Some(sst)),
                Right => st.insert_right(Some(sst)),
                Stop => unreachable!(),
            };
            sst = st;
        }
        match root_action {
            Left => root.insert_left(Some(sst)),
            Right => root.insert_right(Some(sst)),
            Stop => unreachable!(),
        };
    }
}
