extern crate binary_tree;

use binary_tree::count::CountTree;

const TOTAL: usize = 1 << 16;

fn main() {
    let mut ct = CountTree::new();
    // no magic
    for i in 0..TOTAL {
        ct.insert(i/2, i);
    }
    println!("{:#?}", ct.get(2048));
}
