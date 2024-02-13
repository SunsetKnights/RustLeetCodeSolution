use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
#[allow(unused)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
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
        let mut curr_node = Rc::clone(&root);
        for idx in 1..data.len() {
            if idx & 1 == 1 {
                curr_node = queue.pop_front().unwrap();
            }
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

    /**
     * 872. 叶子相似的树
     * 如果有两棵二叉树的叶值序列是相同，那么我们就认为它们是 叶相似 的。
     * 如果给定的两个根结点分别为 root1 和 root2 的树是叶相似的，则返回 true；否则返回 false 。
     */
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn dfs(node: Rc<RefCell<TreeNode>>, leaf: &mut Vec<i32>) {
            let mut is_leaf = true;
            if node.borrow().left.is_some() {
                is_leaf = false;
                dfs(Rc::clone(node.borrow().left.as_ref().unwrap()), leaf);
            }
            if node.borrow().right.is_some() {
                is_leaf = false;
                dfs(Rc::clone(node.borrow().right.as_ref().unwrap()), leaf);
            }
            if is_leaf {
                leaf.push(node.borrow().val);
            }
        }
        let mut leaf_1 = Vec::new();
        let mut leaf_2 = Vec::new();
        dfs(root1.unwrap(), &mut leaf_1);
        dfs(root2.unwrap(), &mut leaf_2);
        leaf_1 == leaf_2
    }

    /**
     * 993. 二叉树的堂兄弟节点
     * 在二叉树中，根节点位于深度 0 处，每个深度为 k 的节点的子节点位于深度 k+1 处。
     * 如果二叉树的两个节点深度相同，但 父节点不同 ，则它们是一对堂兄弟节点。
     * 我们给出了具有唯一值的二叉树的根节点 root ，以及树中两个不同节点的值 x 和 y 。
     * 只有与值 x 和 y 对应的节点是堂兄弟节点时，才返回 true 。否则，返回 false。
     */
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        fn dfs(
            from: i32,
            layer: i32,
            node: Rc<RefCell<TreeNode>>,
            x: i32,
            y: i32,
            x_info: &mut (i32, i32),
            y_info: &mut (i32, i32),
        ) {
            if node.borrow().val == x {
                *x_info = (layer, from);
            }
            if node.borrow().val == y {
                *y_info = (layer, from);
            }
            if node.borrow().left.is_some() {
                dfs(
                    node.borrow().val,
                    layer + 1,
                    Rc::clone(node.borrow().left.as_ref().unwrap()),
                    x,
                    y,
                    x_info,
                    y_info,
                );
            }
            if node.borrow().right.is_some() {
                dfs(
                    node.borrow().val,
                    layer + 1,
                    Rc::clone(node.borrow().right.as_ref().unwrap()),
                    x,
                    y,
                    x_info,
                    y_info,
                );
            }
        }
        let mut x_info = (-1, 0);
        let mut y_info = (-1, 0);
        dfs(0, 0, root.unwrap(), x, y, &mut x_info, &mut y_info);
        x_info.0 == y_info.0 && x_info.1 != y_info.1
    }

    /**
     * 236. 二叉树的最近公共祖先
     * 给定一个二叉树, 找到该树中两个指定节点的最近公共祖先。
     * 百度百科中最近公共祖先的定义为：
     * “对于有根树 T 的两个节点 p、q，最近公共祖先表示为一个节点 x，满足 x 是 p、q 的祖先
     * 且 x 的深度尽可能大（一个节点也可以是它自己的祖先）。”
     */
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            node: Rc<RefCell<TreeNode>>,
            p: i32,
            q: i32,
            ret: &mut Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if ret.is_some() {
                return false;
            }
            let mut curr_find = false;
            if node.borrow().val == p || node.borrow().val == q {
                curr_find = true;
            }
            let mut left_find = false;
            if node.borrow().left.is_some() {
                left_find = dfs(Rc::clone(node.borrow().left.as_ref().unwrap()), p, q, ret);
            }
            let mut right_find = false;
            if node.borrow().right.is_some() {
                right_find = dfs(Rc::clone(node.borrow().right.as_ref().unwrap()), p, q, ret);
            }
            if (curr_find && (left_find || right_find)) || (left_find && right_find) {
                *ret = Some(node);
            }
            curr_find || left_find || right_find
        }
        let mut ret = None;
        dfs(
            root.unwrap(),
            p.unwrap().borrow().val,
            q.unwrap().borrow().val,
            &mut ret,
        );
        ret
    }

    /**
     * 235. 二叉搜索树的最近公共祖先
     * 给定一个二叉搜索树, 找到该树中两个指定节点的最近公共祖先。
     * 百度百科中最近公共祖先的定义为：
     * “对于有根树 T 的两个结点 p、q，最近公共祖先表示为一个结点 x，
     * 满足 x 是 p、q 的祖先且 x 的深度尽可能大（一个节点也可以是它自己的祖先）。”
     */
    pub fn lowest_common_ancestor_search_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // p < q
        fn dfs(node: Rc<RefCell<TreeNode>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if node.borrow().val < p {
                dfs(Rc::clone(node.borrow().right.as_ref().unwrap()), p, q)
            } else if node.borrow().val > q {
                dfs(Rc::clone(node.borrow().left.as_ref().unwrap()), p, q)
            } else {
                Some(node)
            }
        }
        let mut p = p.unwrap().borrow().val;
        let mut q = q.unwrap().borrow().val;
        if p > q {
            std::mem::swap(&mut p, &mut q);
        }
        dfs(root.unwrap(), p, q)
    }

    /**
     * 987. 二叉树的垂序遍历
     * 给你二叉树的根结点 root ，请你设计算法计算二叉树的 垂序遍历 序列。
     * 对位于 (row, col) 的每个结点而言，其左右子结点分别位于 (row + 1, col - 1) 和 (row + 1, col + 1) 。
     * 树的根结点位于 (0, 0) 。
     * 二叉树的 垂序遍历 从最左边的列开始直到最右边的列结束，按列索引每一列上的所有结点，形成一个按出现位置从上到下排序的有序列表。
     * 如果同行同列上有多个结点，则按结点的值从小到大进行排序。
     * 返回二叉树的 垂序遍历 序列。
     */
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(node: Rc<RefCell<TreeNode>>, row: i32, col: i32, info: &mut Vec<(i32, i32, i32)>) {
            info.push((row, col, node.borrow().val));
            if let Some(left) = node.borrow_mut().left.take() {
                dfs(left, row + 1, col - 1, info);
            }
            if let Some(right) = node.borrow_mut().right.take() {
                dfs(right, row + 1, col + 1, info);
            }
        }
        let mut info = Vec::new();
        dfs(root.unwrap(), 0, 0, &mut info);
        info.sort_by(|a, b| {
            if a.1 != b.1 {
                a.1.cmp(&b.1)
            } else if a.0 != b.0 {
                a.0.cmp(&b.0)
            } else {
                a.2.cmp(&b.2)
            }
        });
        let mut ret = Vec::new();
        let mut curr_col = i32::MAX;
        for i in info {
            if i.1 != curr_col {
                ret.push(Vec::new());
            }
            ret.last_mut().unwrap().push(i.2);
        }
        ret
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solution() {
        let tree_val = [5, 4, 9, 1, 10, i32::MAX, 7];
        let mut root = TreeNode::create_from_vec(tree_val.to_vec());
        root = Solution::replace_value_in_tree(root);
        let result = TreeNode::to_vec(root);
        assert_eq!(
            result,
            vec![0, 0, 0, 7, 7, 11],
            "The results are not as expected."
        );
    }
}
