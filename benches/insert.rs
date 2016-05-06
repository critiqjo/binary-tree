#![feature(test)]

extern crate test;
extern crate binary_tree;

use binary_tree::count::CountTree;
use test::Bencher;

use std::collections::LinkedList;

#[bench]
pub fn insert_at_k_ct(b: &mut Bencher) {
    b.iter(|| {
        let mut ct: CountTree<_> = (0..4000).collect();
        for i in 0..4000 {
            ct.insert(300, i);
        }
    })
}

#[bench]
pub fn insert_at_k_ll(b: &mut Bencher) {
    b.iter(|| {
        let mut ll: LinkedList<_> = (0..4000).collect();
        for i in 0..4000 {
            let mut tail = ll.split_off(300);
            ll.push_back(i);
            ll.append(&mut tail);
        }
    })
}

#[bench]
pub fn insert_at_k_vec(b: &mut Bencher) {
    b.iter(|| {
        let mut v: Vec<_> = (0..4000).collect();
        for i in 0..4000 {
            v.insert(300, i);
        }
    })
}
