//! Counting tree implementation.
//!
//! ## When should you use `CountTree`?
//!
//! - You want to maintain a possibly large unsorted list.
//! - You want to access, modify, insert, and delete elements at arbitrary
//!   position with O(log(n)) time complexity.
//! - You can tolerate O(n log(n)) time-complexity for (not implemented yet):
//!   - splitting at arbitrary position
//!   - truncating the length (complexity unclear)
//!   - appending another list (complexity unclear)
//! - You have less than 4.29 billion (`u32::MAX`) elements!

use std::mem;
use std::iter::FromIterator;

use Node;
use NodeMut;
use BinaryTree;
use WalkAction;
use iter::Iter as GenIter;
use iter::IntoIter as GenIntoIter;

pub type NodePtr<T> = Box<CountNode<T>>;

macro_rules! index_walker {
    ($index:ident, $node:ident, $up_count:ident, $stop:block) => {
        {
            let cur_index = $node.lcount() as usize + $up_count;
            if $index < cur_index {
                Left
            } else if $index == cur_index {
                $stop
                Stop
            } else {
                $up_count = cur_index + 1;
                Right
            }
        }
    }
}

/// Counting tree.
///
/// A balanced binary tree which keeps track of total number of child nodes in
/// each node, so that elements can be inserted and deleted using its in-order
/// index. The algorithm used internally is a variation of [AVL Tree][avlwiki].
/// Time complexities mentioned below are that of worst case scenario (and are
/// of the same order as that of an AVL tree).
///
/// [avlwiki]: https://en.wikipedia.org/wiki/AVL_tree
///
/// # Examples
///
/// ```rust
/// # extern crate binary_tree;
/// # use binary_tree::count::CountTree;
/// # fn main() {
/// let mut ct: CountTree<i32> = CountTree::new();
/// ct.push_front(20);
/// ct.push_front(10);
/// assert_eq!(ct.pop_back().unwrap(), 20);
/// # }
/// ```
///
/// You can also use `collect` to create one from an iterator. This has a time
/// complexity of O(n), which is much more efficient than inserting iteratively.
///
/// ```rust
/// # extern crate binary_tree;
/// # use binary_tree::count::CountTree;
/// # fn main() {
/// let mut ct: CountTree<i32> = (0..100).collect();
/// assert_eq!(ct.remove(32), 32);
/// # }
/// ```
pub struct CountTree<T>(Option<NodePtr<T>>);

impl<T> CountTree<T> {
    fn root_must(&mut self) -> &mut CountNode<T> {
        &mut **self.0.as_mut().unwrap()
    }

    /// Returns an empty `CountTree`
    pub fn new() -> CountTree<T> {
        CountTree(None)
    }

    /// Returns `true` if the tree contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    /// Returns the number elements in the tree. Time complexity: O(1)
    pub fn len(&self) -> usize {
        self.root().map_or(0, |node| node.count as usize)
    }

    /// Clears the tree, dropping all elements iteratively.
    pub fn clear(&mut self) {
        let mut inner = None;
        mem::swap(&mut self.0, &mut inner);
        let _: GenIntoIter<CountNode<T>> = GenIntoIter::new(inner);
    }

    /// Returns the element at the given index, or `None` if index is out of
    /// bounds. Time complexity: O(log(n))
    pub fn get<'a>(&'a self, index: usize) -> Option<&'a T> {
        use WalkAction::*;

        if index >= self.len() {
            None
        } else {
            let mut val = None;
            let mut up_count = 0;
            self.root().unwrap().walk(|node: &'a CountNode<T>| {
                index_walker!(index, node, up_count, {
                    val = Some(node.value());
                })
            });
            assert!(val.is_some());
            val
        }
    }

    /// Returns a mutable reference to the element at the given index, or `None`
    /// if out of bounds. Time complexity: O(log(n))
    pub fn get_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut T> {
        use WalkAction::*;

        if index >= self.len() {
            None
        } else {
            let mut val = None;
            let mut up_count = 0;
            let root = self.root_must();
            root.walk_mut(|node| index_walker!(index, node, up_count, {}),
                          |node: &'a mut CountNode<T>| {
                              val = Some(node.value_mut());
                          });
            assert!(val.is_some());
            val
        }
    }

    /// Inserts an element at the given index. Time complexity: O(log(n))
    ///
    /// ## Panics
    ///
    /// Panics if index is greater than `self.len()`
    pub fn insert(&mut self, index: usize, value: T) {
        use WalkAction::*;

        let len = self.len();
        if index == 0 {
            self.push_front(value);
        } else if index < len {
            let new_node = Box::new(CountNode::new(value));
            let mut up_count = 0;
            let root = self.root_must();
            root.walk_reshape(|node| index_walker!(index, node, up_count, {}),
                              move |node| {
                                  node.insert_before(new_node,
                                                     |node, _| node.rebalance());
                              },
                              |node, _| node.rebalance());
        } else if index == len {
            self.push_back(value);
        } else {
            panic!("index out of bounds!");
        }
    }

    /// Prepends an element at the beginning.
    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(CountNode::new(value));
        if self.is_empty() {
            self.0 = Some(new_node);
        } else {
            self.root_must().walk_reshape(|_| WalkAction::Left,
                                          move |node| {
                                              node.insert_left(Some(new_node));
                                          },
                                          |node, _| node.rebalance());
        }
    }

    /// Appends an element at the end.
    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(CountNode::new(value));
        if self.is_empty() {
            self.0 = Some(new_node);
        } else {
            self.root_must().walk_reshape(|_| WalkAction::Right,
                                          move |node| {
                                              node.insert_right(Some(new_node));
                                          },
                                          |node, _| node.rebalance());
        }
    }

    /// Removes the element at the given index. Time complexity: O(log(n))
    ///
    /// ## Panics
    ///
    /// Panics if index is out of bounds.
    pub fn remove(&mut self, index: usize) -> T {
        use WalkAction::*;

        let len = self.len();
        if index == 0 {
            self.pop_front().expect("Tree is empty!")
        } else if index + 1 < len {
            let mut up_count = 0;
            let root = self.root_must();
            root.walk_extract(|node| index_walker!(index, node, up_count, {}),
                              |node, ret| {
                                  *ret = node.try_remove(|node, _| {
                                      node.rebalance()
                                  });
                              },
                              |node, _| node.rebalance())
                .unwrap()
                .into_value()
        } else if index + 1 == len {
            self.pop_back().unwrap()
        } else {
            panic!("index out of bounds!");
        }
    }

    /// Removes and returns the first element, or `None` if empty.
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else if self.len() == 1 {
            Some(self.0.take().unwrap().into_value())
        } else {
            let root = self.root_must();
            Some(root.walk_extract(|_| WalkAction::Left,
                                   |node, ret| {
                                       if let Some(mut right) = node.detach_right() {
                                           mem::swap(&mut *right, node);
                                           *ret = Some(right);
                                       }
                                   },
                                   |node, _| node.rebalance())
                     .unwrap()
                     .into_value())
        }
    }

    /// Removes and returns the last element, or `None` if empty.
    pub fn pop_back(&mut self) -> Option<T> {
        // FIXME Ewww! Code duplication!
        if self.is_empty() {
            None
        } else if self.len() == 1 {
            Some(self.0.take().unwrap().into_value())
        } else {
            let root = self.root_must();
            Some(root.walk_extract(|_| WalkAction::Right,
                                   |node, ret| {
                                       if let Some(mut left) = node.detach_left() {
                                           mem::swap(&mut *left, node);
                                           *ret = Some(left);
                                       }
                                   },
                                   |node, _| node.rebalance())
                     .unwrap()
                     .into_value())
        }
    }

    // TODO ? iter_mut
    // TODO { O(n) } truncate, append, split_off, retain
}

impl<T> BinaryTree for CountTree<T> {
    type Node = CountNode<T>;

    fn root(&self) -> Option<&Self::Node> {
        self.0.as_ref().map(|nodeptr| &**nodeptr)
    }
}

impl<T> Drop for CountTree<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

fn is_power(v: u32) -> bool {
    if v == 0 {
        false
    } else {
        v & (v - 1) == 0
    }
}

fn exp_floor_log(v: u32) -> u32 {
    if v == 0 || is_power(v) {
        v
    } else {
        let mut efl = v - 1;
        efl |= efl >> 1;
        efl |= efl >> 2;
        efl |= efl >> 4;
        efl |= efl >> 8;
        efl |= efl >> 16;
        efl += 1;
        efl >> 1
    }
}

impl<T> FromIterator<T> for CountTree<T> {
    /// Time complexity: &Theta;(n + log<sup>2</sup>(n))
    fn from_iter<I>(iterable: I) -> Self
        where I: IntoIterator<Item = T>
    {
        use WalkAction::*;

        let mut iter = iterable.into_iter();
        if let Some(item) = iter.next() {
            let mut node = Box::new(CountNode::new(item));
            let mut count = 1;
            for item in iter {
                let mut new_node = Box::new(CountNode::new(item));
                new_node.insert_left(Some(node));
                node = new_node;
                count += 1;
                let rcount = if is_power(count + 1) {
                    count >> 1
                } else {
                    count
                };
                let mut rotate_points = 1;
                while rcount & rotate_points == rotate_points {
                    node.rotate_right().unwrap();
                    rotate_points <<= 1;
                    rotate_points |= 1;
                }
            }
            let balanced_till = exp_floor_log(count + 1) - 1;
            count = node.lcount() + 1; // not needed
            while count > balanced_till {
                node.rotate_right().unwrap();
                node.right
                    .as_mut()
                    .unwrap()
                    .walk_reshape(|node| {
                                      if node.balance_factor() > 1 {
                                          node.rotate_right().unwrap();
                                          Right
                                      } else {
                                          Stop
                                      }
                                  },
                                  |_| (),
                                  |_, _| ());
                count = node.lcount() + 1;
            }
            CountTree(Some(node))
        } else {
            CountTree::new()
        }
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

/// Node of a `CountTree`.
///
/// The only way of getting your hands on a `CountNode` is through
/// [`CountTree::root()`](struct.CountTree.html#method.root) method which
/// returns a shared reference to its root.  Thus `NodeMut` methods are not
/// accessible to users.
pub struct CountNode<T> {
    val: T,
    left: Option<NodePtr<T>>,
    right: Option<NodePtr<T>>,
    count: u32,
    height: u16,
}

impl<T> CountNode<T> {
    fn new(val: T) -> CountNode<T> {
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

    // generalized version of AVL tree balance factor: h(left) - h(right)
    fn balance_factor(&self) -> i32 {
        self.left.as_ref().map_or(-1, |node| node.height as i32) -
            self.right.as_ref().map_or(-1, |node| node.height as i32)
    }

    // AVL tree algorithm
    fn rebalance(&mut self) {
        if self.balance_factor() > 1 {
            self.left.as_mut().map(|node| {
                if node.balance_factor() < 0 {
                    node.rotate_left().unwrap();
                }
            });
            self.rotate_right().unwrap();
        } else if self.balance_factor() < -1 {
            self.right.as_mut().map(|node| {
                if node.balance_factor() > 0 {
                    node.rotate_right().unwrap();
                }
            });
            self.rotate_left().unwrap();
        }
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

    fn value_mut(&mut self) -> &mut T {
        &mut self.val
    }

    fn into_value(self) -> T {
        self.val
    }

    fn left_mut<'a>(&'a mut self) -> Option<&'a mut Self> {
        self.left.as_mut().map(|l| &mut **l)
    }

    fn right_mut<'a>(&'a mut self) -> Option<&'a mut Self> {
        self.right.as_mut().map(|r| &mut **r)
    }
}

#[cfg(test)]
mod tests {
    use BinaryTree;
    use NodeMut;
    use super::CountNode;
    use super::CountTree;
    use test::compute_level;
    use test::Level;

    fn test_nodes() -> Box<CountNode<u32>> {
        let mut cn = Box::new(CountNode::new(7));
        cn.insert_before(Box::new(CountNode::new(8)), |_, _| ());
        cn.insert_before(Box::new(CountNode::new(12)), |_, _| ());
        cn.insert_right(Some(Box::new(CountNode::new(5))));
        cn
    }

    #[test]
    fn custom() {
        let ct = CountTree(Some(test_nodes()));
        assert_eq!(ct.get(0), Some(&8));
        assert_eq!(ct.get(1), Some(&12));
        assert_eq!(ct.get(2), Some(&7));
        assert_eq!(ct.get(3), Some(&5));
        assert_eq!(ct.get(4), None);
        let mut ct = ct;
        ct.get_mut(3).map(|v| *v = 100);
        assert_eq!(ct.get(3), Some(&100));
    }

    #[test]
    fn counting() {
        let cn = test_nodes();
        assert_eq!(cn.lcount(), 2);
        assert_eq!(cn.rcount(), 1);
        assert_eq!(cn.count, 4);
        assert_eq!(cn.height, 2);
    }

    #[test]
    fn rebalance() {
        let mut cn = test_nodes();
        assert_eq!(cn.balance_factor(), 1);
        cn.detach_right();
        cn.rebalance();
        assert_eq!(cn.balance_factor(), 0);
        assert_eq!(compute_level(&*cn, 1), Level::Balanced(2));
        let ct = CountTree(Some(cn));
        assert_eq!(ct.get(0), Some(&8));
        assert_eq!(ct.get(1), Some(&12));
        assert_eq!(ct.get(2), Some(&7));
        assert_eq!(ct.get(3), None);
    }

    #[test]
    fn insert() {
        let mut ct = CountTree::new();
        assert_eq!(ct.get(0), None);
        ct.insert(0, 2);
        ct.insert(0, 3);
        ct.insert(0, 4);
        ct.insert(0, 5);
        ct.insert(0, 6);
        assert_eq!(ct.get(0), Some(&6));
        assert_eq!(ct.get(1), Some(&5));
        assert_eq!(ct.get(2), Some(&4));
        ct.insert(0, 7);
        assert_eq!(ct.get(4), Some(&3));
        assert_eq!(ct.get(5), Some(&2));
        assert_eq!(ct.root().unwrap().height, 2);
        assert_eq!(compute_level(ct.root().unwrap(), 1), Level::Balanced(3));
        ct.insert(6, 1);
        assert_eq!(ct.get(6), Some(&1));
        assert_eq!(ct.root().unwrap().height, 3);
        assert_eq!(compute_level(ct.root().unwrap(), 1), Level::Balanced(4));
    }

    #[test]
    fn from_iter() {
        let ct: CountTree<_> = (0..63).collect();
        let root = ct.root().unwrap();
        assert_eq!(root.height, 5);
        assert_eq!(compute_level(root, 0), Level::Balanced(6));

        let ct: CountTree<_> = (0..94).collect();
        let root = ct.root().unwrap();
        assert_eq!(root.balance_factor(), -1);
        assert_eq!(root.height, 6);
        assert_eq!(compute_level(root, 1), Level::Balanced(7));
    }

    #[test]
    fn remove() {
        let mut ct: CountTree<_> = (0..94).collect();
        for i in 0..20 {
            assert_eq!(ct.remove(64), 64 + i);
            assert!(compute_level(ct.root().unwrap(), 1).is_balanced());
        }
    }
}
