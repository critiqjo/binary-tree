#![feature(test)]

extern crate rand;
extern crate test;
extern crate binary_tree;

use binary_tree::count::CountTree;
use rand::{Rand, Rng, StdRng, SeedableRng, thread_rng};
use std::env;
use std::str::FromStr;
use test::Bencher;

const TOTAL: usize = 7936;

fn get_seed() -> usize {
    match env::var("INSERT_SEED") {
        Ok(val) => usize::from_str(&*val).unwrap(),
        Err(_) => {
            let seed = usize::rand(&mut thread_rng());
            env::set_var("INSERT_SEED", seed.to_string());
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
        for i in 0..TOTAL {
            ct.insert(rng.gen_range(0, i + 1), i);
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
