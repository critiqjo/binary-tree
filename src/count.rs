use std::mem;

use Node;
use NodeMut;
use BinaryTree;
use iter::NodeIter;

pub trait Countable {
    fn count(&self) -> u64;
}

impl Countable for u64 {
    fn count(&self) -> u64 {
        *self
    }
}

pub struct CountTree<T>(CountNode<T>);

impl<T: Countable> CountTree<T> {
    pub fn new(val: T) -> CountTree<T> {
        CountTree(CountNode::new(val))
    }
}

impl<T: Countable> BinaryTree for CountTree<T> {
    type Node = CountNode<T>;

    fn root(&self) -> &Self::Node {
        &self.0
    }
}

impl<'a, T> IntoIterator for &'a CountTree<T>
    where T: Countable
{
    type Item = &'a T;
    type IntoIter = impl Iterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIter::new(&self.0).map(|n| &n.val)
    }
}

pub struct CountNode<T> {
    val: T,
    left: Option<Box<CountNode<T>>>,
    right: Option<Box<CountNode<T>>>,
    left_sum: u64, // excluding val.count()
    right_sum: u64, // ditto
}

impl<T: Countable> CountNode<T> {
    pub fn new(val: T) -> CountNode<T> {
        CountNode {
            val: val,
            left: None,
            right: None,
            left_sum: 0,
            right_sum: 0,
        }
    }

    pub fn total_count(&self) -> u64 {
        self.left_sum + self.val.count() + self.right_sum
    }
}

impl<T: Countable> Node for CountNode<T> {
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

impl<T: Countable> NodeMut for CountNode<T> {
    type NodePtr = Box<CountNode<T>>;

    fn detach_left(&mut self) -> Option<Self::NodePtr> {
        self.left_sum = 0;
        self.left.take()
    }

    fn detach_right(&mut self) -> Option<Self::NodePtr> {
        self.right_sum = 0;
        self.right.take()
    }

    fn insert_left(&mut self, mut tree: Option<Self::NodePtr>) -> Option<Self::NodePtr> {
        self.left_sum = tree.as_ref().map_or(0, |tree| tree.total_count());
        mem::swap(&mut self.left, &mut tree);
        tree
    }

    fn insert_right(&mut self, mut tree: Option<Self::NodePtr>) -> Option<Self::NodePtr> {
        self.right_sum = tree.as_ref().map_or(0, |tree| tree.total_count());
        mem::swap(&mut self.right, &mut tree);
        tree
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
        assert_eq!(ct.left_sum, 20);
        assert_eq!(ct.right_sum, 5);
        assert_eq!(ct.total_count(), 32);
    }
}
