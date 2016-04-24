//! Counting trees.
//!
//! ## When should you use CountTree?
//!
//! - You want to maintain a possibly large unsorted list.
//! - You want to access, modify, insert, and delete elements at arbitrary
//!   position without incurring O(n) time complexity.

use std::mem;

use Node;
use NodeMut;
use BinaryTree;
use iter::NodeIter;

pub type NodePtr<T> = Box<CountNode<T>>;

pub struct CountTree<T>(CountNode<T>);

impl<T> CountTree<T> {
    pub fn new(val: T) -> CountTree<T> {
        CountTree(CountNode::new(val))
    }

    // TODO get, get_mut, insert, delete, len, {push|pop}_{front|back}
    // TODO [?] clear, is_empty, iter_mut
    // TODO [hard] truncate, append, split_off, impl FromIterator, retain
}

impl<T> BinaryTree for CountTree<T> {
    type Node = CountNode<T>;

    fn root(&self) -> &Self::Node {
        &self.0
    }
}

impl<'a, T> IntoIterator for &'a CountTree<T> {
    type Item = &'a T;
    type IntoIter = NodeIter<'a, CountNode<T>>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIter::new(&self.0)
    }
}

pub struct CountNode<T> {
    val: T,
    left: Option<NodePtr<T>>,
    right: Option<NodePtr<T>>,
    lcount: u64, // not counting oneself
    rcount: u64, // ditto
}

impl<T> CountNode<T> {
    pub fn new(val: T) -> CountNode<T> {
        CountNode {
            val: val,
            left: None,
            right: None,
            lcount: 0,
            rcount: 0,
        }
    }

    pub fn count(&self) -> u64 {
        self.lcount + 1 + self.rcount
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
        self.lcount = 0;
        self.left.take()
    }

    fn detach_right(&mut self) -> Option<Self::NodePtr> {
        self.rcount = 0;
        self.right.take()
    }

    fn insert_left(&mut self, mut tree: Option<Self::NodePtr>) -> Option<Self::NodePtr> {
        self.lcount = tree.as_ref().map_or(0, |tree| tree.count());
        mem::swap(&mut self.left, &mut tree);
        tree
    }

    fn insert_right(&mut self, mut tree: Option<Self::NodePtr>) -> Option<Self::NodePtr> {
        self.rcount = tree.as_ref().map_or(0, |tree| tree.count());
        mem::swap(&mut self.right, &mut tree);
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
        assert_eq!(ct.lcount, 2);
        assert_eq!(ct.rcount, 1);
        assert_eq!(ct.count(), 4);
    }
}
