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
pub fn remove_at_random_ct(b: &mut Bencher) {
    let seed = get_seed();
    let rng = StdRng::from_seed(&[seed]);
    b.iter(|| {
        let mut rng = rng.clone();
        let mut ct: CountTree<_> = (0..TOTAL).collect();
        for i in 0..TOTAL {
            ct.remove(rng.gen_range(0, TOTAL - i));
        }
    });
}

#[bench]
pub fn remove_at_random_ll(b: &mut Bencher) {
    let seed = get_seed();
    let rng = StdRng::from_seed(&[seed]);
    b.iter(|| {
        let mut rng = rng.clone();
        let mut ll: LinkedList<_> = (0..TOTAL).collect();
        for i in 0..TOTAL {
            let mut tail = ll.split_off(rng.gen_range(0, TOTAL - i));
            tail.pop_front();
            ll.append(&mut tail);
        }
    });
}

#[bench]
pub fn remove_at_random_vec(b: &mut Bencher) {
    let seed = get_seed();
    let rng = StdRng::from_seed(&[seed]);
    b.iter(|| {
        let mut rng = rng.clone();
        let mut v: Vec<_> = (0..TOTAL).collect();
        for i in 0..TOTAL {
            v.remove(rng.gen_range(0, TOTAL - i));
        }
    });
}
