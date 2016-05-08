//! Data structures and algorithms for testing purposes.

use std::mem;
use std::cmp;

use Node;
use NodeMut;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Balanced(u32),
    Imbalanced(u32),
}

impl Level {
    pub fn is_balanced(self) -> bool {
        if let Level::Balanced(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_u32(self) -> u32 {
        match self {
            Level::Balanced(e) |
            Level::Imbalanced(e) => e,
        }
    }
}

/// Recursively calculate the level of this node and check whether it is
/// balanced.
///
/// `level = height + 1`. Recursive, hence risk of stack blow up depending on
/// the height of the tree! The node is considered balanced if, at every node,
/// the difference in levels of the child nodes is not greater than `tolerance`.
pub fn compute_level<N: Node>(node: &N, tolerance: u32) -> Level {
    use test::Level::*;

    let llevel = node.left().map_or(Balanced(0), |n| compute_level(n, tolerance));
    let rlevel = node.right().map_or(Balanced(0), |n| compute_level(n, tolerance));

    if llevel.is_balanced() && rlevel.is_balanced() {
        let max = cmp::max(llevel.as_u32(), rlevel.as_u32());
        let min = cmp::min(llevel.as_u32(), rlevel.as_u32());
        if max - min > tolerance {
            Imbalanced(max + 1)
        } else {
            Balanced(max + 1)
        }
    } else {
        Imbalanced(cmp::max(llevel.as_u32(), rlevel.as_u32()) + 1)
    }
}

#[derive(Debug)]
/// A minimal `Node` implementation.
///
/// ## When should you use `TestNode`?
///
/// You should not use `TestNode` for anything, except may be to get to know what
/// a binary tree is!
pub struct TestNode<T> {
    pub val: T,
    pub left: Option<Box<TestNode<T>>>,
    pub right: Option<Box<TestNode<T>>>,
}

impl<T> TestNode<T> {
    pub fn new(val: T) -> TestNode<T> {
        TestNode {
            val: val,
            left: None,
            right: None,
        }
    }
}

impl<T> Node for TestNode<T> {
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

impl<T> NodeMut for TestNode<T> {
    type NodePtr = Box<TestNode<T>>;

    fn detach_left(&mut self) -> Option<Self::NodePtr> {
        self.left.take()
    }

    fn detach_right(&mut self) -> Option<Self::NodePtr> {
        self.right.take()
    }

    fn insert_left(&mut self, mut st: Option<Self::NodePtr>) -> Option<Self::NodePtr> {
        mem::swap(&mut self.left, &mut st);
        st
    }

    fn insert_right(&mut self, mut st: Option<Self::NodePtr>) -> Option<Self::NodePtr> {
        mem::swap(&mut self.right, &mut st);
        st
    }

    fn value_owned(self) -> T {
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
    use super::TestNode;
    use Node;
    use NodeMut;

    fn new_node<T>(val: T) -> Box<TestNode<T>> {
        Box::new(TestNode::new(val))
    }

    fn test_tree() -> TestNode<u32> {
        TestNode {
            val: 20,
            left: Some(new_node(10)),
            right: Some(Box::new(TestNode {
                val: 30,
                left: Some(new_node(25)),
                right: None,
            })),
        }
    }

    #[test]
    fn rotate() {
        let mut tt = test_tree();

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

        let mut tt = test_tree();
        let mut steps = vec![Right, Left, Stop];
        {
            let mut step_iter = steps.drain(..);
            tt.walk_mut(|_| step_iter.next().unwrap(),
                        |st| assert_eq!(st.val, 25),
                        |st, action| {
                            match action {
                                Right => assert_eq!(st.val, 20),
                                Left => assert_eq!(st.val, 30),
                                Stop => unreachable!(),
                            }
                        });
        }
        assert_eq!(steps.len(), 0);
    }

    #[test]
    fn remove() {
        let mut tt = test_tree();
        let tn = tt.right.as_mut().unwrap().try_remove(|_, _| ());
        assert_eq!(tn.unwrap().value(), &30);
        assert_eq!(tt.right.as_ref().unwrap().value(), &25);
        let mut tt2 = test_tree();
        {
            let right = tt2.right.as_mut().unwrap();
            right.right = Some(new_node(40));
        }
        let tn = tt2.right.as_mut().unwrap().try_remove(|_, _| ());
        assert_eq!(tn.unwrap().value(), &30);
        assert_eq!(tt.right.as_ref().unwrap().value(), &25);
    }

    #[test]
    fn stack_blow() {
        use iter::IntoIter;
        let mut pt = new_node(20);
        for _ in 0..200000 {
            let mut pt2 = new_node(20);
            pt2.insert_left(Some(pt));
            pt = pt2;
        }
        // comment out the line below to observe a stack overflow
        let _: IntoIter<TestNode<_>> = IntoIter::new(Some(pt));
    }
}
