mod inner_mutable_tree;
use std::{cell::RefCell, rc::Rc};

use inner_mutable_tree::*;

fn main() {
    let tree1_val = [3, 5, 1, 6, 2, 0, 8, i32::MAX, i32::MAX, 7, 4];
    let root = TreeNode::create_from_vec(tree1_val.to_vec());
    assert_eq!(
        Solution::lowest_common_ancestor(
            root,
            Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            Some(Rc::new(RefCell::new(TreeNode::new(4))))
        )
        .unwrap()
        .borrow()
        .val,
        5,
        "The results are not as expected."
    );
}
