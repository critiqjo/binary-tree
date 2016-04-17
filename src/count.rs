use BinaryTree;

pub trait Countable {
    fn count(&self) -> u64 {
        1
    }
}

pub struct CountTree<T> {
    val: T,
    left_sum: u64,
    left: Option<Box<CountTree<T>>>,
    right: Option<Box<CountTree<T>>>,
}

impl<T: Countable> CountTree<T> {
    pub fn new(val: T) -> CountTree<T> {
        let count = val.count();
        CountTree {
            val: val,
            left_sum: count,
            left: None,
            right: None,
        }
    }

    pub fn total_count(&self) -> u64 {
        let mut total = self.left_sum;
        let mut subtree = self.right.as_ref();
        while let Some(st) = subtree {
            total += st.left_sum;
            subtree = st.right.as_ref();
        }
        total
    }
}

impl<T: Countable> BinaryTree for CountTree<T> {
    type Value = T;
    type Subtree = Box<CountTree<T>>;

    fn detach_left(&mut self) -> Option<Self::Subtree> {
        if let Some(subtree) = self.left.take() {
            assert!(self.left_sum > subtree.value().count());
            self.left_sum -= subtree.value().count();
            Some(subtree)
        } else {
            None
        }
    }

    fn detach_right(&mut self) -> Option<Self::Subtree> {
        self.right.take()
    }

    fn insert_left(&mut self, subtree: Option<Self::Subtree>) -> Option<Self::Subtree> {
        let old = self.detach_left();
        if let Some(ref subtree) = subtree {
            self.left_sum += subtree.value().count();
        }
        self.left = subtree;
        old
    }

    fn insert_right(&mut self, subtree: Option<Self::Subtree>) -> Option<Self::Subtree> {
        let old = self.detach_right();
        self.right = subtree;
        old
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
    fn rotate() {
        let mut ct = CountTree::new(Value(7));
        ct.insert_left(Some(box CountTree::new(Value(10))));
        ct.insert_right(Some(box CountTree::new(Value(5))));
        assert_eq!(ct.left_sum, 17);
        assert_eq!(ct.total_count(), 22);
    }
}
