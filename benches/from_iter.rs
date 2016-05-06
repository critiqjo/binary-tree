#![feature(test)]

extern crate test;
extern crate binary_tree;

use binary_tree::count::CountTree;
use test::Bencher;

use std::collections::LinkedList;

#[bench]
fn from_iter_ct(b: &mut Bencher) {
    b.iter(|| {
        (0..2000).collect::<CountTree<_>>();
    })
}

#[bench]
fn from_iter_ll(b: &mut Bencher) {
    b.iter(|| {
        (0..2000).collect::<LinkedList<_>>();
    })
}

#[bench]
fn from_iter_vec(b: &mut Bencher) {
    b.iter(|| {
        (0..2000).collect::<Vec<_>>();
    })
}
