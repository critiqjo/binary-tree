#![feature(plugin)]
#![plugin(quickcheck_macros)]
#![cfg(feature="quickcheck")]

extern crate quickcheck;
extern crate binary_tree;

use binary_tree::BinaryTree;
use binary_tree::count::CountTree;
use binary_tree::test::compute_level;
use quickcheck::TestResult;

macro_rules! qc_assert {
    ($cond:expr) => {
        if !($cond) {
            return TestResult::failed();
        }
    }
}


#[quickcheck]
fn qc_insert(mut ct: CountTree<usize>, index: usize) -> TestResult {
    if index > ct.len() {
        return TestResult::discard();
    }

    let prev = if index > 0 {
        ct.get(index - 1).map(|v| *v)
    } else {
        None
    };
    let this = ct.len(); // the value to be inserted
    let next = ct.get(index).map(|v| *v);

    ct.insert(index, this);
    if let Some(prev) = prev {
        qc_assert!(prev == *ct.get(index - 1).unwrap());
    }
    qc_assert!(this == *ct.get(index).unwrap());
    if let Some(next) = next {
        qc_assert!(next == *ct.get(index + 1).unwrap());
    }
    let level = compute_level(ct.root().unwrap(), 1);
    qc_assert!(level.is_balanced());

    TestResult::passed()
}

#[quickcheck]
fn qc_remove(mut ct: CountTree<usize>, index: usize) -> TestResult {
    if index + 1 > ct.len() {
        return TestResult::discard();
    }

    let prev = if index > 0 {
        ct.get(index - 1).map(|v| *v)
    } else {
        None
    };
    let next = ct.get(index + 1).map(|v| *v);

    ct.remove(index);
    if let Some(prev) = prev {
        qc_assert!(prev == *ct.get(index - 1).unwrap());
    }
    if let Some(next) = next {
        qc_assert!(next == *ct.get(index).unwrap());
    }
    if let Some(root) = ct.root() {
        let level = compute_level(root, 1);
        qc_assert!(level.is_balanced());
    }

    TestResult::passed()
}
