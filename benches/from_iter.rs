#![feature(test)]

extern crate test;
extern crate binary_tree;

use binary_tree::count::CountTree;
use test::Bencher;

use std::collections::LinkedList;

const TOTAL: usize = 4096;

#[bench]
fn from_iter_ct(b: &mut Bencher) {
    b.iter(|| {
        (0..TOTAL).collect::<CountTree<_>>();
    })
}

#[bench]
fn from_iter_ll(b: &mut Bencher) {
    b.iter(|| {
        (0..TOTAL).collect::<LinkedList<_>>();
    })
}

#[bench]
fn from_iter_vec(b: &mut Bencher) {
    b.iter(|| {
        (0..TOTAL).collect::<Vec<_>>();
    })
}
