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
//! - You have less than 4.29 billion elements!

use std::mem;

use Node;
use NodeMut;
use BinaryTree;
use iter::Iter as GenIter;
use iter::IntoIter as GenIntoIter;

pub type NodePtr<T> = Box<CountNode<T>>;

pub struct CountTree<T>(Option<NodePtr<T>>);

impl<T> CountTree<T> {
    pub fn new() -> CountTree<T> {
        CountTree(None)
    }

    pub fn len(&self) -> usize {
        self.root().map_or(0, |node| node.count as usize)
    }

    pub fn get<'a>(&'a self, index: usize) -> Option<&'a T> {
        use WalkAction::*;

        if index >= self.len() {
            None
        } else {
            let mut val = None;
            let mut up_count = 0;
            self.root().unwrap().walk(|cn: &'a CountNode<T>| {
                let cur_index = cn.lcount() as usize + up_count;
                if index < cur_index {
                    Left
                } else if index == cur_index {
                    val = Some(cn.value());
                    Stop
                } else {
                    up_count = cur_index + 1;
                    Right
                }
            });
            assert!(val.is_some());
            val
        }
    }

    // TODO get_mut, insert, delete, {push|pop}_{front|back}
    // TODO ? clear, is_empty, iter_mut
    // TODO { O(n) } truncate, append, split_off, impl FromIterator, retain
}

impl<T> BinaryTree for CountTree<T> {
    type Node = CountNode<T>;

    fn root(&self) -> Option<&Self::Node> {
        self.0.as_ref().map(|nodeptr| &**nodeptr)
    }
}

// prevent the unlikely event of stack overflow
impl<T> Drop for CountTree<T> {
    fn drop(&mut self) {
        let mut inner = None;
        mem::swap(&mut self.0, &mut inner);
        let _: GenIntoIter<CountNode<T>> = GenIntoIter::new(inner);
    }
}

impl<'a, T> IntoIterator for &'a CountTree<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: GenIter::new(self.root()),
            remaining: self.len(),
        }
    }
}

pub struct Iter<'a, T: 'a> {
    inner: GenIter<'a, CountNode<T>>,
    remaining: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.remaining > 0 {
            self.remaining -= 1;
        }
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

impl<T> IntoIterator for CountTree<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(mut self) -> Self::IntoIter {
        let len = self.len();
        let mut inner = None;
        mem::swap(&mut self.0, &mut inner);
        IntoIter {
            inner: GenIntoIter::new(inner),
            remaining: len,
        }
    }
}

pub struct IntoIter<T> {
    inner: GenIntoIter<CountNode<T>>,
    remaining: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.remaining > 0 {
            self.remaining -= 1;
        }
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

pub struct CountNode<T> {
    val: T,
    left: Option<NodePtr<T>>,
    right: Option<NodePtr<T>>,
    count: u32,
    height: u16,
}

impl<T> CountNode<T> {
    pub fn new(val: T) -> CountNode<T> {
        CountNode {
            val: val,
            left: None,
            right: None,
            count: 1,
            height: 0,
        }
    }

    fn lcount(&self) -> u32 {
        self.left.as_ref().map_or(0, |tree| tree.count)
    }

    fn rcount(&self) -> u32 {
        self.right.as_ref().map_or(0, |tree| tree.count)
    }

    fn update_stats(&mut self) {
        use std::cmp::max;
        self.count = self.lcount() + self.rcount() + 1;
        self.height = max(self.left.as_ref().map_or(0, |tree| tree.height),
                          self.right.as_ref().map_or(0, |tree| tree.height));
        if self.count > 1 {
            self.height += 1;
        }
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
        self.update_stats();
        tree
    }

    fn detach_right(&mut self) -> Option<Self::NodePtr> {
        let tree = self.right.take();
        self.update_stats();
        tree
    }

    fn insert_left(&mut self, mut tree: Option<Self::NodePtr>) -> Option<Self::NodePtr> {
        mem::swap(&mut self.left, &mut tree);
        self.update_stats();
        tree
    }

    fn insert_right(&mut self, mut tree: Option<Self::NodePtr>) -> Option<Self::NodePtr> {
        mem::swap(&mut self.right, &mut tree);
        self.update_stats();
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
    use super::CountTree;

    #[test]
    fn counting() {
        let mut cn = Box::new(CountNode::new(7));
        let mut cn_l = Box::new(CountNode::new(8));
        cn_l.insert_right(Some(Box::new(CountNode::new(12))));
        cn.insert_left(Some(cn_l));
        cn.insert_right(Some(Box::new(CountNode::new(5))));
        assert_eq!(cn.lcount(), 2);
        assert_eq!(cn.rcount(), 1);
        assert_eq!(cn.count, 4);
        assert_eq!(cn.height, 2);
        let ct = CountTree(Some(cn));
        assert_eq!(ct.get(0), Some(&8));
        assert_eq!(ct.get(1), Some(&12));
        assert_eq!(ct.get(2), Some(&7));
        assert_eq!(ct.get(3), Some(&5));
        assert_eq!(ct.get(4), None);
    }
}
