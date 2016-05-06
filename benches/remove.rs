#![feature(test)]

extern crate test;
extern crate binary_tree;

use binary_tree::count::CountTree;
use test::Bencher;

use std::collections::LinkedList;

#[bench]
pub fn remove_at_k_ct(b: &mut Bencher) {
    b.iter(|| {
        let mut ct: CountTree<_> = (0..6000).collect();
        for _ in 0..3000 {
            ct.remove(300);
        }
    })
}

#[bench]
pub fn remove_at_k_ll(b: &mut Bencher) {
    b.iter(|| {
        let mut ll: LinkedList<_> = (0..6000).collect();
        for _ in 0..3000 {
            let mut tail = ll.split_off(300);
            ll.pop_back();
            ll.append(&mut tail);
        }
    })
}

#[bench]
pub fn remove_at_k_vec(b: &mut Bencher) {
    b.iter(|| {
        let mut v: Vec<_> = (0..6000).collect();
        for _ in 0..3000 {
            v.remove(300);
        }
    })
}
