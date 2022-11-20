mod tree;
use tree::*;

fn main() {
    let mut tree = Tree::Empty;
    tree.add(1);
    tree.add(2);
    tree.add(3);

    println!("Tree: {:?}", tree);
}
