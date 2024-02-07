use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn create_from_vec(data: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // i32::MAX means null
        if data.is_empty() || data[0] == i32::MAX {
            return None;
        }
        let mut queue = VecDeque::new();
        let root = Rc::new(RefCell::new(Self::new(data[0])));
        queue.push_back(Rc::clone(&root));
        for idx in 1..data.len() {
            let curr_node = queue.pop_front().unwrap();
            if data[idx] != i32::MAX {
                let child = Rc::new(RefCell::new(TreeNode::new(data[idx])));
                queue.push_back(Rc::clone(&child));
                if idx & 1 == 1 {
                    curr_node.borrow_mut().left = Some(child);
                } else {
                    curr_node.borrow_mut().right = Some(child);
                }
            }
        }
        Some(root)
    }

    pub fn to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = Vec::new();
        if let Some(root) = root {
            let mut queue = VecDeque::new();
            ret.push(root.borrow().val);
            queue.push_back(root);
            while let Some(node) = queue.pop_front() {
                ret.push(node.borrow().val);
                if node.borrow().left.is_some() {
                    queue.push_back(node.borrow_mut().left.take().unwrap());
                }
                if node.borrow().right.is_some() {
                    queue.push_back(node.borrow_mut().right.take().unwrap());
                }
            }
        }
        ret
    }
}

pub struct Solution;

#[allow(unused)]
impl Solution {
    /**
     * 538. 把二叉搜索树转换为累加树
     * 给出二叉 搜索 树的根节点，该树的节点值各不相同，请你将其转换为累加树（Greater Sum Tree），使每个节点 node 的新值等于原树中大于或等于 node.val 的值之和。
     * 提醒一下，二叉搜索树满足下列约束条件：
     *     节点的左子树仅包含键 小于 节点键的节点。
     *     节点的右子树仅包含键 大于 节点键的节点。
     *     左右子树也必须是二叉搜索树。
     */
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn bfs(root: &Rc<RefCell<TreeNode>>, bigger_sum: &mut i32) {
            if root.borrow().right.is_some() {
                bfs(root.borrow_mut().right.as_ref().unwrap(), bigger_sum);
            }
            *bigger_sum += root.borrow().val;
            root.borrow_mut().val = *bigger_sum;
            if root.borrow().left.is_some() {
                bfs(root.borrow_mut().left.as_ref().unwrap(), bigger_sum);
            }
        }
        let mut bigger_sum = 0;
        if root.is_some() {
            bfs(root.as_ref().unwrap(), &mut bigger_sum)
        }
        root
    }

    /**
     * 1457. 二叉树中的伪回文路径
     * 给你一棵二叉树，每个节点的值为 1 到 9 。我们称二叉树中的一条路径是 「伪回文」的，当它满足：路径经过的所有节点值的排列中，存在一个回文序列。
     * 请你返回从根到叶子节点的所有路径中 伪回文 路径的数目。
     */
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

    /**
     * 2641. 二叉树的堂兄弟节点 II
     * 给你一棵二叉树的根 root ，请你将每个节点的值替换成该节点的所有 堂兄弟节点值的和 。
     * 如果两个节点在树中有相同的深度且它们的父节点不同，那么它们互为 堂兄弟 。
     * 请你返回修改值之后，树的根 root 。
     * 注意，一个节点的深度指的是从树根节点到这个节点经过的边数。
     */
    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue = VecDeque::new();
        queue.push_back((
            Rc::clone(root.as_ref().unwrap()),
            root.as_ref().unwrap().borrow().val,
        ));
        while !queue.is_empty() {
            let level_sum: i32 = queue.iter().map(|(node, _)| node.borrow().val).sum();
            let mut new_queue = VecDeque::new();
            for (node, bro_sum) in queue {
                node.borrow_mut().val = level_sum - bro_sum;
                let mut child_sum = 0;
                if node.borrow().left.is_some() {
                    child_sum += node.borrow().left.as_ref().unwrap().borrow().val;
                }
                if node.borrow().right.is_some() {
                    child_sum += node.borrow().right.as_ref().unwrap().borrow().val;
                }
                if node.borrow().left.is_some() {
                    new_queue
                        .push_back((Rc::clone(node.borrow().left.as_ref().unwrap()), child_sum));
                }
                if node.borrow().right.is_some() {
                    new_queue
                        .push_back((Rc::clone(node.borrow().right.as_ref().unwrap()), child_sum));
                }
            }
            queue = new_queue;
        }
        root
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solution() {
        let tree_val = [3, 5, 1, 6, 2, 9, 8, i32::MAX, i32::MAX, 7, 4];
        let mut root = TreeNode::create_from_vec(tree_val.to_vec());
        root = Solution::replace_value_in_tree(root);
        let result = TreeNode::to_vec(root);
        assert_eq!(result, vec![1, 2, 5], "The results are not as expected.");
    }
}
