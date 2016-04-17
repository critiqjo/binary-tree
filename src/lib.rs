#![feature(box_syntax)]

use std::ops::DerefMut;

pub mod cow;

/// Generic methods on binary trees. Calling any of these methods directly on a
/// self-balancing binary tree can make it imbalanced.
pub trait BinaryTree: Sized {
    type Subtree: Sized + DerefMut<Target=Self>;

    /// Try to detach the left sub-tree
    fn detach_left(&mut self) -> Option<Self::Subtree>;

    /// Try to detach the right sub-tree
    fn detach_right(&mut self) -> Option<Self::Subtree>;

    /// Replace the left subtree with `tree` and return the old one.
    fn insert_left(&mut self, tree: Option<Self::Subtree>) -> Option<Self::Subtree>;

    /// Replace the right subtree with `tree` and return the old one.
    fn insert_right(&mut self, tree: Option<Self::Subtree>) -> Option<Self::Subtree>;

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
        type Subtree = Box<TestTree>;

        fn detach_left(&mut self) -> Option<Self::Subtree> {
            self.left.take()
        }
        fn detach_right(&mut self) -> Option<Self::Subtree> {
            self.right.take()
        }
        fn insert_left(&mut self, tree: Option<Self::Subtree>) -> Option<Self::Subtree> {
            let old = self.left.take();
            self.left = tree;
            old
        }
        fn insert_right(&mut self, tree: Option<Self::Subtree>) -> Option<Self::Subtree> {
            let old = self.right.take();
            self.right = tree;
            old
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
        assert_eq!(tt_lrot.val,                         30);
        assert_eq!(tt_lrot.left.as_ref().unwrap().val,  20);
        assert_eq!(tt_lrot.left.as_ref().unwrap()
                          .left.as_ref().unwrap().val,  10);
        assert_eq!(tt_lrot.left.as_ref().unwrap()
                          .right.as_ref().unwrap().val, 25);

        let tt: Box<TestTree> = TestTree::rotate_right(tt_lrot).unwrap();
        assert_eq!(tt.val,                         20);
        assert_eq!(tt.left.as_ref().unwrap().val,  10);
        assert_eq!(tt.right.as_ref().unwrap().val, 30);
        assert_eq!(tt.right.as_ref().unwrap()
                     .left.as_ref().unwrap().val,  25);
    }
}
