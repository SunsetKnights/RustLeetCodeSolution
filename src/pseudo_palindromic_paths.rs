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
        let mut flag_vector: u32 = 0;
        if let Some(r) = root {
            if r.borrow().left.is_none() && r.borrow().right.is_none() {
                return 1;
            }
            flag_vector ^= 1 << r.borrow().val as u32;
            stack.push(r);
        }

        while stack.len() > 0 {
            // put all left node to stack
            let mut curr = stack.last().unwrap().borrow_mut();
            let mut next = None;
            if curr.left.is_some() {
                next = Some(curr.left.take().unwrap());
            } else if curr.right.is_some() {
                next = Some(curr.left.take().unwrap());
            }
            if next.is_none() {
                drop(curr);
                let val = stack.pop().unwrap().borrow().val;
                flag_vector ^= 1 << val;
            } else {
                let next = next.unwrap();
                let val = next.borrow().val as u32;
                flag_vector ^= 1 << val;
                if next.borrow().left.is_none() && next.borrow().right.is_none() {
                    if flag_vector == 0 || flag_vector & (flag_vector - 1) == 0 {
                        ret += 1;
                    }
                    flag_vector ^= 1 << val;
                } else {
                    drop(curr);
                    stack.push(next);
                }
            }
        }
        ret
    }
}
