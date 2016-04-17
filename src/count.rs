use std::mem;
use BinaryTree;

pub trait Countable {
    fn count(&self) -> u64 {
        1
    }
}

pub struct CountTree<T> {
    val: T,
    left: Option<Box<CountTree<T>>>,
    right: Option<Box<CountTree<T>>>,
    left_sum: u64, // excluding val.count()
    right_sum: u64, // ditto
}

impl<T: Countable> CountTree<T> {
    pub fn new(val: T) -> CountTree<T> {
        CountTree {
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

impl<T: Countable> BinaryTree for CountTree<T> {
    type Value = T;
    type Subtree = Box<CountTree<T>>;

    fn detach_left(&mut self) -> Option<Self::Subtree> {
        self.left_sum = 0;
        self.left.take()
    }

    fn detach_right(&mut self) -> Option<Self::Subtree> {
        self.right_sum = 0;
        self.right.take()
    }

    fn insert_left(&mut self, mut tree: Option<Self::Subtree>) -> Option<Self::Subtree> {
        self.left_sum = tree.as_ref().map_or(0, |tree| tree.total_count());
        mem::swap(&mut self.left, &mut tree);
        tree
    }

    fn insert_right(&mut self, mut tree: Option<Self::Subtree>) -> Option<Self::Subtree> {
        self.right_sum = tree.as_ref().map_or(0, |tree| tree.total_count());
        mem::swap(&mut self.right, &mut tree);
        tree
    }

    fn value(&self) -> &T {
        &self.val
    }
}

#[cfg(test)]
mod tests {
    use BinaryTree;
    use super::CountTree;
    use super::Countable;

    struct Value(u8);
    impl Countable for Value {
        fn count(&self) -> u64 {
            self.0 as u64
        }
    }

    #[test]
    fn counting() {
        let mut ct = CountTree::new(Value(7));
        let mut tt_l = CountTree::new(Value(8));
        tt_l.insert_right(Some(box CountTree::new(Value(12))));
        ct.insert_left(Some(box tt_l));
        ct.insert_right(Some(box CountTree::new(Value(5))));
        assert_eq!(ct.left_sum, 20);
        assert_eq!(ct.right_sum, 5);
        assert_eq!(ct.total_count(), 32);
    }
}
