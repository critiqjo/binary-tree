#![feature(box_syntax)]

use std::mem;
use std::ops::DerefMut;

pub mod cow;
pub mod count;

/// Generic methods on binary trees. Calling any of these methods directly on a
/// self-balancing binary tree may make it imbalanced.
pub trait BinaryTree: Sized {
    type Value;
    type Subtree: Sized + DerefMut<Target=Self>;

    /// Try to detach the left sub-tree
    fn detach_left(&mut self) -> Option<Self::Subtree>;

    /// Try to detach the right sub-tree
    fn detach_right(&mut self) -> Option<Self::Subtree>;

    /// Replace the left subtree with `tree` and return the old one.
    fn insert_left(&mut self, tree: Option<Self::Subtree>) -> Option<Self::Subtree>;

    /// Replace the right subtree with `tree` and return the old one.
    fn insert_right(&mut self, tree: Option<Self::Subtree>) -> Option<Self::Subtree>;

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

// Walks down the tree by detaching subtrees, then reattaches them back.
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

#[cfg(test)]
mod tests {
    use super::BinaryTree;
    use std::mem;

    #[derive(Debug)]
    struct TestTree {
        val: u32,
        left: Option<Box<TestTree>>,
        right: Option<Box<TestTree>>,
    }

    impl TestTree {
        fn new(val: u32) -> TestTree {
            TestTree {
                val: val,
                left: None,
                right: None,
            }
        }
    }

    impl BinaryTree for TestTree {
        type Value = u32;
        type Subtree = Box<TestTree>;

        fn detach_left(&mut self) -> Option<Self::Subtree> {
            self.left.take()
        }
        fn detach_right(&mut self) -> Option<Self::Subtree> {
            self.right.take()
        }
        fn insert_left(&mut self, mut st: Option<Self::Subtree>) -> Option<Self::Subtree> {
            mem::swap(&mut self.left, &mut st);
            st
        }
        fn insert_right(&mut self, mut st: Option<Self::Subtree>) -> Option<Self::Subtree> {
            mem::swap(&mut self.right, &mut st);
            st
        }
        fn value(&self) -> &u32 {
            &self.val
        }
    }

    #[test]
    fn rotate() {
        let mut tt = TestTree::new(20);
        tt.insert_left(Some(box TestTree::new(10)));
        let mut tt_r = TestTree::new(30);
        tt_r.insert_left(Some(box TestTree::new(25)));
        tt.insert_right(Some(box tt_r));

        tt.rotate_left().unwrap();
        assert_eq!(*tt.value(),                    30);
        assert_eq!(tt.left.as_ref().unwrap().val,  20);
        assert_eq!(tt.left.as_ref().unwrap()
                     .left.as_ref().unwrap().val,  10);
        assert_eq!(tt.left.as_ref().unwrap()
                     .right.as_ref().unwrap().val, 25);

        tt.rotate_right().unwrap();
        assert_eq!(*tt.value(),                    20);
        assert_eq!(tt.left.as_ref().unwrap().val,  10);
        assert_eq!(tt.right.as_ref().unwrap().val, 30);
        assert_eq!(tt.right.as_ref().unwrap()
                     .left.as_ref().unwrap().val,  25);
    }

    #[test]
    fn walk() {
        use WalkAction::*;

        let mut tt = TestTree::new(20);
        tt.insert_left(Some(box TestTree::new(10)));
        let mut tt_r = TestTree::new(30);
        tt_r.insert_left(Some(box TestTree::new(25)));
        tt.insert_right(Some(box tt_r));

        let mut steps = vec![Right, Left, Stop];
        {
            let mut step_iter = steps.drain(..);
            super::walk_mut(&mut tt, |st| {
                let action = step_iter.next().unwrap();
                match action {
                    Right => assert_eq!(st.val, 20),
                    Left => assert_eq!(st.val, 30),
                    Stop => assert_eq!(st.val, 25),
                }
                action
            });
        }
        assert_eq!(steps.len(), 0);
    }
}
