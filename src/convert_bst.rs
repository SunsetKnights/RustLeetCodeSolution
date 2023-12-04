pub struct Solution;
use std::cell::RefCell;
/**
 * 538. 把二叉搜索树转换为累加树
 * 给出二叉 搜索 树的根节点，该树的节点值各不相同，请你将其转换为累加树（Greater Sum Tree），使每个节点 node 的新值等于原树中大于或等于 node.val 的值之和。
 * 提醒一下，二叉搜索树满足下列约束条件：
 *     节点的左子树仅包含键 小于 节点键的节点。
 *     节点的右子树仅包含键 大于 节点键的节点。
 *     左右子树也必须是二叉搜索树。
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
use std::rc::Rc;
impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut bigger_sum = 0;
        if root.is_some() {
            Solution::bfs(root.as_ref().unwrap(), &mut bigger_sum)
        }
        root
    }

    fn bfs(root: &Rc<RefCell<TreeNode>>, bigger_sum: &mut i32) {
        if root.borrow().right.is_some() {
            Solution::bfs(root.borrow_mut().right.as_ref().unwrap(), bigger_sum);
        }
        *bigger_sum += root.borrow().val;
        root.borrow_mut().val = *bigger_sum;
        if root.borrow().left.is_some() {
            Solution::bfs(root.borrow_mut().left.as_ref().unwrap(), bigger_sum);
        }
    }
}
