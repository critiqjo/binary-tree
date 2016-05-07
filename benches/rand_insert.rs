#![feature(test)]

extern crate rand;
extern crate test;
extern crate binary_tree;

use binary_tree::count::CountTree;
use rand::{Rng, StdRng};
use test::Bencher;

#[bench]
pub fn insert_at_random_ct(b: &mut Bencher) {
    let mut rng = StdRng::new().unwrap();
    b.iter(|| {
        let mut ct = CountTree::new();
        ct.insert(0, 0);
        for i in 1..7936 {
            ct.insert(rng.gen_range(0, i), i);
        }
    })
}

#[bench]
pub fn insert_at_random_vec(b: &mut Bencher) {
    let mut rng = StdRng::new().unwrap();
    b.iter(|| {
        let mut v = Vec::new();
        v.insert(0, 0);
        for i in 1..7936 {
            v.insert(rng.gen_range(0, i), i);
        }
    })
}
