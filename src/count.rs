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
//!
//! ## Benchmarks
//!
//! The constants in the complexity bounds are not small enough to make an
//! immediate switch from `Vec` to `CountTree`. In the benchmarks below, `*_ct`
//! indicate `CountTree`, `*_ll` indicate `LinkedList` and `*_vec` indicate
//! `Vec`. `from_iter_*` indicate the performance of using `collect()`,
//! `insert_at_random_*` indicate that of inserting N elements at random
//! positions, and `remove_at_random_*` indicate that of first creating an N
//! sized object using `collect()` and then removing all N elements at random
//! one-by-one. See `benches` directory for more details.
//!
//! ### Bench: N=2048
//!
//! ```sh
//!      Running target/release/from_iter-86b0c3c534a106e8
//!
//! running 3 tests
//! test from_iter_ct  ... bench:     316,440 ns/iter (+/- 92,914)
//! test from_iter_ll  ... bench:     299,755 ns/iter (+/- 2,096)
//! test from_iter_vec ... bench:       2,839 ns/iter (+/- 24)
//!
//!      Running target/release/insert-892f694b6341f60d
//!
//! running 3 tests
//! test insert_at_random_ct  ... bench:   1,569,694 ns/iter (+/- 239,724)
//! test insert_at_random_ll  ... bench:   2,882,318 ns/iter (+/- 20,555)
//! test insert_at_random_vec ... bench:     570,018 ns/iter (+/- 3,710)
//!
//!      Running target/release/remove-12ee8f5093c08f36
//!
//! running 3 tests
//! test remove_at_random_ct  ... bench:   1,800,295 ns/iter (+/- 5,761)
//! test remove_at_random_ll  ... bench:   2,702,035 ns/iter (+/- 22,632)
//! test remove_at_random_vec ... bench:     568,502 ns/iter (+/- 3,626)
//! ```
//!
//! ### Bench: N=4096
//!
//! ```sh
//!      Running target/release/from_iter-86b0c3c534a106e8
//!
//! running 3 tests
//! test from_iter_ct  ... bench:     698,944 ns/iter (+/- 25,819)
//! test from_iter_ll  ... bench:     579,370 ns/iter (+/- 12,161)
//! test from_iter_vec ... bench:       5,582 ns/iter (+/- 26)
//!
//!      Running target/release/insert-892f694b6341f60d
//!
//! running 3 tests
//! test insert_at_random_ct  ... bench:   3,495,019 ns/iter (+/- 123,200)
//! test insert_at_random_ll  ... bench:  14,470,666 ns/iter (+/- 39,605)
//! test insert_at_random_vec ... bench:   1,896,108 ns/iter (+/- 3,925)
//!
//!      Running target/release/remove-12ee8f5093c08f36
//!
//! running 3 tests
//! test remove_at_random_ct  ... bench:   3,966,049 ns/iter (+/- 25,852)
//! test remove_at_random_ll  ... bench:  11,981,076 ns/iter (+/- 77,152)
//! test remove_at_random_vec ... bench:   1,909,054 ns/iter (+/- 5,475)
//! ```
//!
//! ### Bench: N=8192
//!
//! ```sh
//!      Running target/release/from_iter-86b0c3c534a106e8
//!
//! running 3 tests
//! test from_iter_ct  ... bench:   1,422,694 ns/iter (+/- 224,526)
//! test from_iter_ll  ... bench:   1,190,954 ns/iter (+/- 17,328)
//! test from_iter_vec ... bench:      11,487 ns/iter (+/- 52)
//!
//!      Running target/release/insert-892f694b6341f60d
//!
//! running 3 tests
//! test insert_at_random_ct  ... bench:   7,651,408 ns/iter (+/- 232,136)
//! test insert_at_random_ll  ... bench:  67,522,968 ns/iter (+/- 508,089)
//! test insert_at_random_vec ... bench:   8,062,135 ns/iter (+/- 46,386)
//!
//!      Running target/release/remove-12ee8f5093c08f36
//!
//! running 3 tests
//! test remove_at_random_ct  ... bench:   8,972,611 ns/iter (+/- 99,882)
//! test remove_at_random_ll  ... bench:  50,513,436 ns/iter (+/- 161,649)
//! test remove_at_random_vec ... bench:   8,166,272 ns/iter (+/- 35,268)
//! ```
//!
//! ### Conclusion
//!
//! In short, if you want to maintiain a list of type `T` such that:
//!
//! ```sh
//! [number of elements] * [size of T] > 256 KB
//! ```
//!
//! then `CountTree` might be a good choice, otherwise you are better off using
//! `Vec`.

use std::mem;
use std::iter::FromIterator;
use std::fmt::{self, Debug};

#[cfg(feature="quickcheck")]
use quickcheck::{Arbitrary, Gen};

use Node;
use NodeMut;
use BinaryTree;
use WalkAction;
use cow::ArcCow;
use iter::Iter as GenIter;
use iter::IntoIter as GenIntoIter;
use unbox::Unbox;

pub type NodePtr<T> = ArcCow<CountNode<T>>;

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
#[derive(Clone)]
pub struct CountTree<T: Clone>(Option<NodePtr<T>>);

impl<T: Clone> CountTree<T> {
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
    pub fn get(&self, index: usize) -> Option<&T> {
        use WalkAction::*;

        if index >= self.len() {
            None
        } else {
            let mut val = None;
            let mut up_count = 0;
            self.root().unwrap().walk(|node| {
                index_walker!(index, node, up_count, {
                    val = Some(node.value());
                })
            });
            debug_assert!(val.is_some());
            val
        }
    }

    /// Returns a mutable reference to the element at the given index, or `None`
    /// if out of bounds. Time complexity: O(log(n))
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        use WalkAction::*;

        if index >= self.len() {
            None
        } else {
            let mut val = None;
            let mut up_count = 0;
            let root = self.root_must();
            root.walk_mut(|node| index_walker!(index, node, up_count, {}),
                          |node| val = Some(node.value_mut()));
            debug_assert!(val.is_some());
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
            let new_node = ArcCow::new(CountNode::new(value));
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
        let new_node = ArcCow::new(CountNode::new(value));
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
        let new_node = ArcCow::new(CountNode::new(value));
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
                                  *ret = node.try_remove(|node, _| node.rebalance());
                              },
                              |node, _| node.rebalance())
                .map(|p| p.unbox())
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
            Some(self.0.take().map(|p| p.unbox()).unwrap().into_value())
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
                     .map(|p| p.unbox())
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
            Some(self.0.take().map(|p| p.unbox()).unwrap().into_value())
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
                     .map(|p| p.unbox())
                     .unwrap()
                     .into_value())
        }
    }

    // TODO ? iter_mut
    // TODO { O(n) } truncate, append, split_off, retain
}

impl<T: Clone> BinaryTree for CountTree<T> {
    type Node = CountNode<T>;

    fn root(&self) -> Option<&Self::Node> {
        self.0.as_ref().map(|nodeptr| &**nodeptr)
    }
}

impl<T: Clone> Debug for CountTree<T>
    where T: Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut ds = f.debug_struct("CountTree");
        if let Some(ref root) = self.0 {
            ds.field("_count", &root.count);
            ds.field("_height", &root.height);
            ds.field("_inner", &DebugPrefix("^", root));
        } else {
            ds.field("_count", &0);
            ds.field("_height", &0);
            ds.field("_inner", &DebugPrefix("^", &()));
        }
        ds.finish()
    }
}

impl<T: Clone> Drop for CountTree<T> {
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

impl<T: Clone> FromIterator<T> for CountTree<T> {
    /// Time complexity: &Theta;(n + log<sup>2</sup>(n))
    fn from_iter<I>(iterable: I) -> Self
        where I: IntoIterator<Item = T>
    {
        use WalkAction::*;

        let mut iter = iterable.into_iter();
        if let Some(item) = iter.next() {
            let mut node = ArcCow::new(CountNode::new(item));
            let mut count = 1;
            for item in iter {
                let mut new_node = ArcCow::new(CountNode::new(item));
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

impl<'a, T: Clone> IntoIterator for &'a CountTree<T> {
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

impl<T: Clone> IntoIterator for CountTree<T> {
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

pub struct IntoIter<T: Clone> {
    inner: GenIntoIter<CountNode<T>>,
    remaining: usize,
}

impl<T: Clone> Iterator for IntoIter<T> {
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

impl<T: Clone> ExactSizeIterator for IntoIter<T> {}

/// Node of a `CountTree`.
///
/// The only way of getting your hands on a `CountNode` is through
/// [`CountTree::root()`](struct.CountTree.html#method.root) method which
/// returns a shared reference to its root.  Thus `NodeMut` methods are not
/// accessible to users.
#[derive(Clone)]
pub struct CountNode<T> {
    val: T,
    left: Option<NodePtr<T>>,
    right: Option<NodePtr<T>>,
    count: u32,
    height: u16,
}

impl<T: Clone> CountNode<T> {
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

    fn into_value(self) -> T {
        debug_assert!(self.count == 1, "count = {}", self.count);
        self.val
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

impl<T: Clone> NodeMut for CountNode<T> {
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

    fn into_parts(self) -> (T, Option<Self::NodePtr>, Option<Self::NodePtr>) {
        (self.val, self.left, self.right)
    }

    fn left_mut(&mut self) -> Option<&mut Self> {
        self.left.as_mut().map(|l| &mut **l)
    }

    fn right_mut(&mut self) -> Option<&mut Self> {
        self.right.as_mut().map(|r| &mut **r)
    }
}

struct DebugPrefix<'a, 'b, T: 'b>(&'a str, &'b T);

impl<'a, 'b, T> Debug for DebugPrefix<'a, 'b, T>
    where T: Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(f.write_str(self.0));
        self.1.fmt(f)
    }
}

impl<T> Debug for CountNode<T>
    where T: Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut dt = f.debug_tuple("");
        dt.field(&self.val);
        if let Some(ref left) = self.left {
            dt.field(&DebugPrefix("L", left));
        }
        if let Some(ref right) = self.right {
            dt.field(&DebugPrefix("R", right));
        }
        dt.finish()
    }
}

#[cfg(feature="quickcheck")]
impl Arbitrary for CountTree<usize> {
    fn arbitrary<G: Gen>(g: &mut G) -> CountTree<usize> {
        let size = { let s = g.size(); g.gen_range(0, s) };
        let mut ct = CountTree::new();
        for i in 0..size {
            ct.insert(g.gen_range(0, i + 1), i);
        }
        ct
    }

    fn shrink(&self) -> Box<Iterator<Item=CountTree<usize>>> {
        Box::new(quickcheck::Shrinker::new(self))
    }
}

#[cfg(feature="quickcheck")]
pub mod quickcheck {
    use super::CountTree;
    use BinaryTree;

    #[derive(Clone, Copy)]
    enum ShrinkerState {
        Value,
        Left,
        Right,
        End,
    }

    pub struct Shrinker {
        inner: CountTree<usize>,
        state: ShrinkerState,
    }

    impl Shrinker {
        pub fn new(inner: &CountTree<usize>) -> Shrinker {
            Shrinker {
                inner: inner.clone(),
                state: ShrinkerState::Value,
            }
        }
    }

    impl Iterator for Shrinker {
        type Item = CountTree<usize>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.inner.0.is_none() {
                None
            } else {
                use self::ShrinkerState::*;
                let root = self.inner.root().unwrap();
                match self.state {
                    Value => {
                        let mut ct = CountTree::new();
                        if root.count > 1 {
                            ct.push_back(root.val);
                            self.state = Left;
                        } else {
                            self.state = End;
                        }
                        Some(ct)
                    }
                    Left => {
                        self.state = Right;
                        Some(CountTree(root.left.clone()))
                    }
                    Right => {
                        self.state = End;
                        Some(CountTree(root.right.clone()))
                    }
                    End => {
                        None
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use BinaryTree;
    use NodeMut;
    use super::CountNode;
    use super::CountTree;
    use cow::ArcCow;
    use test::compute_level;
    use test::Level;

    fn test_nodes() -> ArcCow<CountNode<u32>> {
        let mut cn = ArcCow::new(CountNode::new(7));
        cn.insert_before(ArcCow::new(CountNode::new(8)), |_, _| ());
        cn.insert_before(ArcCow::new(CountNode::new(12)), |_, _| ());
        cn.insert_right(Some(ArcCow::new(CountNode::new(5))));
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
