extern crate binary_tree;

use binary_tree::count::CountTree;

const TOTAL: usize = 1 << 16;

fn main() {
    let mut ct = CountTree::new();
    ct.insert(0, 0); // magical insert
    for i in 1..TOTAL { // starts at 1
        ct.insert(i/2, i);
    }
    println!("{:#?}", ct.get(2048));
}
