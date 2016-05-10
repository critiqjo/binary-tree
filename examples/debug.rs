extern crate binary_tree;

use binary_tree::count::CountTree;

fn main() {
    let ct: CountTree<_> = (0..0).collect();
    println!("{:?}", ct);

    let mut ct: CountTree<_> = (0..7).collect();
    ct.push_back(8);
    println!("{:#?}", ct);
}
