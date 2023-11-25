pub mod pseudo_palindromic_paths;
use std::{cell::RefCell, rc::Rc};

use pseudo_palindromic_paths::*;

fn main() {
    let a = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    let c = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    let mut b = TreeNode::new(9);
    b.left = a;
    b.right = c;
    let result = Solution::pseudo_palindromic_paths(Some(Rc::new(RefCell::new(b))));
    println!("{:?}", result);
}
