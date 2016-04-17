use std::mem;

use BinaryTree;

#[derive(Debug)]
pub struct PlainTree<T> {
    val: T,
    left: Option<Box<PlainTree<T>>>,
    right: Option<Box<PlainTree<T>>>,
}

impl<T> PlainTree<T> {
    pub fn new(val: T) -> PlainTree<T> {
        PlainTree {
            val: val,
            left: None,
            right: None,
        }
    }
}

impl<T> BinaryTree for PlainTree<T> {
    type Value = T;
    type Subtree = Box<PlainTree<T>>;

    fn left(&self) -> Option<&Self::Subtree> {
        self.left.as_ref()
    }

    fn right(&self) -> Option<&Self::Subtree> {
        self.right.as_ref()
    }

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

    fn value(&self) -> &T {
        &self.val
    }
}

#[cfg(test)]
mod tests {
    use super::PlainTree;
    use BinaryTree;
    use walk_mut;

    #[test]
    fn rotate() {
        let mut tt = PlainTree::new(20);
        tt.insert_left(Some(Box::new(PlainTree::new(10))));
        let mut tt_r = PlainTree::new(30);
        tt_r.insert_left(Some(Box::new(PlainTree::new(25))));
        tt.insert_right(Some(Box::new(tt_r)));

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

        let mut tt = PlainTree::new(20);
        tt.insert_left(Some(Box::new(PlainTree::new(10))));
        let mut tt_r = PlainTree::new(30);
        tt_r.insert_left(Some(Box::new(PlainTree::new(25))));
        tt.insert_right(Some(Box::new(tt_r)));

        let mut steps = vec![Right, Left, Stop];
        {
            let mut step_iter = steps.drain(..);
            walk_mut(&mut tt, |st| {
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
