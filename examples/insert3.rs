extern crate binary_tree;

use binary_tree::count::CountTree;

const TOTAL: usize = 1 << 16;

fn main() {
    let mut ct = CountTree::new();
    ct.insert(0, 0); ct.remove(0); // magical no-op
    for i in 0..TOTAL {
        ct.insert(i/2, i);
    }
    println!("{:#?}", ct.get(2048));
}
