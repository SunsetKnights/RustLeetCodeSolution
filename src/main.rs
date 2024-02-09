mod inner_mutable_tree;
use std::{cell::RefCell, rc::Rc};

use inner_mutable_tree::*;

fn main() {
    let tree1_val = [6, 2, 8, 0, 4, 7, 9, i32::MAX, i32::MAX, 3, 5];
    let root = TreeNode::create_from_vec(tree1_val.to_vec());
    assert_eq!(
        Solution::lowest_common_ancestor_search_tree(
            root,
            Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            Some(Rc::new(RefCell::new(TreeNode::new(8))))
        )
        .unwrap()
        .borrow()
        .val,
        6,
        "The results are not as expected."
    );
}
