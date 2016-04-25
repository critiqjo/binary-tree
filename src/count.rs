//! Counting trees.
//!
//! ## When should you use CountTree?
//!
//! - You want to maintain a possibly large unsorted list.
//! - You want to access, modify, insert, and delete elements at arbitrary
//!   position with O(logn) time complexity.
//! - You can tolerate O(n logn) time-complexity for:
//!   - splitting at arbitrary position
//!   - truncating the length
//!   - appending another list

use std::mem;

use Node;
use NodeMut;
use BinaryTree;
use iter::{Iter, IntoIter};

pub type NodePtr<T> = Box<CountNode<T>>;

pub struct CountTree<T>(NodePtr<T>);

impl<T> CountTree<T> {
    pub fn new(val: T) -> CountTree<T> {
        CountTree(Box::new(CountNode::new(val)))
    }

    // TODO get, get_mut, insert, delete, len, {push|pop}_{front|back}
    // TODO ? clear, is_empty, iter_mut
    // TODO { O(n) } truncate, append, split_off, impl FromIterator, retain
}

impl<T> BinaryTree for CountTree<T> {
    type Node = CountNode<T>;

    fn root(&self) -> &Self::Node {
        &*self.0
    }
}

impl<'a, T> IntoIterator for &'a CountTree<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, CountNode<T>>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self.root())
    }
}

impl<T> IntoIterator for CountTree<T> {
    type Item = T;
    type IntoIter = IntoIter<CountNode<T>>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.0)
    }
}

pub struct CountNode<T> {
    val: T,
    left: Option<NodePtr<T>>,
    right: Option<NodePtr<T>>,
    total: u64,
}

impl<T> CountNode<T> {
    pub fn new(val: T) -> CountNode<T> {
        CountNode {
            val: val,
            left: None,
            right: None,
            total: 1,
        }
    }

    pub fn count(&self) -> u64 {
        self.total
    }

    fn lcount(&self) -> u64 {
        self.left.as_ref().map_or(0, |tree| tree.count())
    }

    fn rcount(&self) -> u64 {
        self.right.as_ref().map_or(0, |tree| tree.count())
    }

    fn update_total(&mut self) {
        self.total = self.lcount() + self.rcount() + 1;
    }
}

impl<T> Node for CountNode<T> {
    type Value = T;

    fn left(&self) -> Option<&Self> {
        self.left.as_ref().map(|st| &**st)
    }

    fn right(&self) -> Option<&Self> {
        self.right.as_ref().map(|st| &**st)
    }

    fn value(&self) -> &T {
        &self.val
    }
}

impl<T> NodeMut for CountNode<T> {
    type NodePtr = NodePtr<T>;

    fn detach_left(&mut self) -> Option<Self::NodePtr> {
        let tree = self.left.take();
        self.update_total();
        tree
    }

    fn detach_right(&mut self) -> Option<Self::NodePtr> {
        let tree = self.right.take();
        self.update_total();
        tree
    }

    fn insert_left(&mut self, mut tree: Option<Self::NodePtr>) -> Option<Self::NodePtr> {
        mem::swap(&mut self.left, &mut tree);
        self.update_total();
        tree
    }

    fn insert_right(&mut self, mut tree: Option<Self::NodePtr>) -> Option<Self::NodePtr> {
        mem::swap(&mut self.right, &mut tree);
        self.update_total();
        tree
    }

    fn value_owned(self) -> T {
        self.val
    }
}

#[cfg(test)]
mod tests {
    use NodeMut;
    use super::CountNode;

    #[test]
    fn counting() {
        let mut ct = CountNode::new(7);
        let mut ct_l = CountNode::new(8);
        ct_l.insert_right(Some(Box::new(CountNode::new(12))));
        ct.insert_left(Some(Box::new(ct_l)));
        ct.insert_right(Some(Box::new(CountNode::new(5))));
        assert_eq!(ct.lcount(), 2);
        assert_eq!(ct.rcount(), 1);
        assert_eq!(ct.count(), 4);
    }
}
