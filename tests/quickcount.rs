#![feature(plugin)]
#![plugin(quickcheck_macros)]
#![cfg(feature="quickcheck")]

extern crate quickcheck;
extern crate binary_tree;

use binary_tree::BinaryTree;
use binary_tree::count::CountTree;
use binary_tree::test::compute_level;
use quickcheck::TestResult;

#[quickcheck]
fn ct_insert(mut ct: CountTree<usize>, index: usize) -> TestResult {
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
    let mut ok = true;
    if let Some(prev) = prev {
        ok = ok && prev == *ct.get(index - 1).unwrap();
    }
    ok = ok && this == *ct.get(index).unwrap();
    if let Some(next) = next {
        ok = ok && next == *ct.get(index + 1).unwrap();
    }
    ok = ok && compute_level(ct.root().unwrap(), 1).is_balanced();
    TestResult::from_bool(ok)
}
