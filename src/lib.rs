#![feature(box_syntax)]

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

    /// Consumes the tree, rotates it left if right subtree exists, otherwise
    /// returns the original with an error.
    fn rotate_left(mut this: Self::Subtree) -> Result<Self::Subtree, Self::Subtree> {
        if let Some(mut new_root) = this.detach_right() {
            let mid = new_root.detach_left();
            this.insert_right(mid);
            new_root.insert_left(Some(this));
            Ok(new_root)
        } else {
            Err(this)
        }
    }

    /// Consumes the tree, rotates it right if left subtree exists, otherwise
    /// returns the original with an error.
    fn rotate_right(mut this: Self::Subtree) -> Result<Self::Subtree, Self::Subtree> {
        if let Some(mut new_root) = this.detach_left() {
            let mid = new_root.detach_right();
            this.insert_left(mid);
            new_root.insert_right(Some(this));
            Ok(new_root)
        } else {
            Err(this)
        }
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

        let tt_lrot: Box<TestTree> = TestTree::rotate_left(box tt).unwrap();
        assert_eq!(*tt_lrot.value(),                    30);
        assert_eq!(tt_lrot.left.as_ref().unwrap().val,  20);
        assert_eq!(tt_lrot.left.as_ref().unwrap()
                          .left.as_ref().unwrap().val,  10);
        assert_eq!(tt_lrot.left.as_ref().unwrap()
                          .right.as_ref().unwrap().val, 25);

        let tt: Box<TestTree> = TestTree::rotate_right(tt_lrot).unwrap();
        assert_eq!(*tt.value(),                    20);
        assert_eq!(tt.left.as_ref().unwrap().val,  10);
        assert_eq!(tt.right.as_ref().unwrap().val, 30);
        assert_eq!(tt.right.as_ref().unwrap()
                     .left.as_ref().unwrap().val,  25);
    }
}
