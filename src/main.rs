pub mod tree_ancestor;
use tree_ancestor::*;

fn main() {
    let test = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
    test.get_kth_ancestor(3, 1);
    test.get_kth_ancestor(5, 2);
    test.get_kth_ancestor(6, 3);
}
