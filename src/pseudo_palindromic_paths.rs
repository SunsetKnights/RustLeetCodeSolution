pub struct Solution {}
/**
 * 1457. 二叉树中的伪回文路径
 * 给你一棵二叉树，每个节点的值为 1 到 9 。我们称二叉树中的一条路径是 「伪回文」的，当它满足：路径经过的所有节点值的排列中，存在一个回文序列。
 * 请你返回从根到叶子节点的所有路径中 伪回文 路径的数目。
 */

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ret = 0;
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        if root.is_some() {
            stack.push(Rc::clone(&root.as_ref().unwrap()));
        }
        let mut flag_vector: u32 = 0;
        while let Some(rc_pointer) = stack.pop() {
            let val = rc_pointer.borrow().val;
            flag_vector ^= 1 << val;
            if flag_vector == 0 || flag_vector & (flag_vector - 1) == 0 {}
        }
        1
    }
    fn get_leaf_quantity(node: Rc<RefCell<TreeNode>>) -> i32 {
    }
}
