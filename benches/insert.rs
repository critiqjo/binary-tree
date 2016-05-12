#![feature(test)]

extern crate binary_tree;
extern crate rand;
extern crate test;

use binary_tree::count::CountTree;
use rand::{Rand, Rng, StdRng, SeedableRng, thread_rng};
use std::env;
use std::str::FromStr;
use std::collections::LinkedList;
use test::Bencher;

const TOTAL: usize = 4096;

fn get_seed() -> usize {
    match env::var("RAND_SEED") {
        Ok(val) => usize::from_str(&*val).unwrap(),
        Err(_) => {
            let seed = usize::rand(&mut thread_rng());
            env::set_var("RAND_SEED", seed.to_string());
            seed
        }
    }
}

#[bench]
pub fn insert_at_random_ct(b: &mut Bencher) {
    let seed = get_seed();
    let rng = StdRng::from_seed(&[seed]);
    b.iter(|| {
        let mut rng = rng.clone();
        let mut ct = CountTree::new();
        ct.insert(0, 0); // magical speed-up
        for i in 1..TOTAL { // starts at 1, not 0
            ct.insert(rng.gen_range(0, i + 1), i);
        }
    });
}

#[bench]
pub fn insert_at_random_ll(b: &mut Bencher) {
    let seed = get_seed();
    let rng = StdRng::from_seed(&[seed]);
    b.iter(|| {
        let mut rng = rng.clone();
        let mut ll = LinkedList::new();
        for i in 0..TOTAL {
            let mut tail = ll.split_off(rng.gen_range(0, i + 1));
            ll.push_back(i);
            ll.append(&mut tail);
        }
    });
}

#[bench]
pub fn insert_at_random_vec(b: &mut Bencher) {
    let seed = get_seed();
    let rng = StdRng::from_seed(&[seed]);
    b.iter(|| {
        let mut rng = rng.clone();
        let mut v = Vec::new();
        for i in 0..TOTAL {
            v.insert(rng.gen_range(0, i + 1), i);
        }
    });
}
