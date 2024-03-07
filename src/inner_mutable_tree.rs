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
        let mut ret = root.unwrap();
        let mut p = p.unwrap().borrow().val;
        let mut q = q.unwrap().borrow().val;
        if p > q {
            std::mem::swap(&mut p, &mut q);
        }
        loop {
            if p > ret.borrow().val {
                let temp = Rc::clone(ret.borrow().right.as_ref().unwrap());
                ret = temp;
            } else if q < ret.borrow().val {
                let temp = Rc::clone(ret.borrow().left.as_ref().unwrap());
                ret = temp;
            } else {
                break;
            }
        }
        Some(ret)
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

    /**
     * 107. 二叉树的层序遍历 II
     * 给你二叉树的根节点 root ，返回其节点值 自底向上的层序遍历 。 （即按从叶子节点所在层到根节点所在的层，逐层从左向右遍历）
     */
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node);
        }
        while queue.len() != 0 {
            let mut curr_level = Vec::with_capacity(queue.len());
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                curr_level.push(node.borrow().val);
                if let Some(left) = node.borrow_mut().left.take() {
                    queue.push_back(left);
                };
                if let Some(right) = node.borrow_mut().right.take() {
                    queue.push_back(right);
                };
            }
            ret.push(curr_level);
        }
        ret.reverse();
        ret
    }

    /**
     * 103. 二叉树的锯齿形层序遍历
     * 给你二叉树的根节点 root ，返回其节点值的 锯齿形层序遍历 。
     * （即先从左往右，再从右往左进行下一层遍历，以此类推，层与层之间交替进行）。
     */
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node);
        }
        while queue.len() != 0 {
            let mut curr_level = Vec::with_capacity(queue.len());
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                curr_level.push(node.borrow().val);
                if let Some(left) = node.borrow_mut().left.take() {
                    queue.push_back(left);
                };
                if let Some(right) = node.borrow_mut().right.take() {
                    queue.push_back(right);
                };
            }
            if ret.len() & 1 == 1 {
                curr_level.reverse();
            }
            ret.push(curr_level);
        }
        ret
    }

    /**
     * 105. 从前序与中序遍历序列构造二叉树
     * 给定两个整数数组 preorder 和 inorder ，其中 preorder 是二叉树的先序遍历，
     * inorder 是同一棵树的中序遍历，请构造二叉树并返回其根节点。
     */
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(preor: &[i32], inor: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if inor.len() == 0 {
                return None;
            }
            let root_idx = (0..inor.len()).find(|idx| inor[*idx] == preor[0]).unwrap();
            let mut root = TreeNode::new(preor[0]);
            root.left = build(&preor[1..=root_idx], &inor[0..root_idx]);
            root.right = build(&preor[root_idx + 1..], &inor[root_idx + 1..]);
            Some(Rc::new(RefCell::new(root)))
        }
        build(&preorder[..], &inorder[..])
    }

    /**
     * 106. 从中序与后序遍历序列构造二叉树
     * 给定两个整数数组 inorder 和 postorder,
     * 其中 inorder 是二叉树的中序遍历，
     * postorder 是同一棵树的后序遍历，请你构造并返回这颗 二叉树 。
     */
    pub fn build_tree_postorder(
        inorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(postor: &[i32], inor: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if inor.len() == 0 {
                return None;
            }
            let root_idx = (0..inor.len())
                .find(|idx| inor[*idx] == postor[postor.len() - 1])
                .unwrap();
            let mut root = TreeNode::new(postor[postor.len() - 1]);
            root.left = build(&postor[0..root_idx], &inor[0..root_idx]);
            root.right = build(&postor[root_idx..postor.len() - 1], &inor[root_idx + 1..]);
            Some(Rc::new(RefCell::new(root)))
        }
        build(&postorder[..], &inorder[..])
    }

    /**
     * 889. 根据前序和后序遍历构造二叉树
     * 给定两个整数数组，preorder 和 postorder ，
     * 其中 preorder 是一个具有 无重复 值的二叉树的前序遍历，
     * postorder 是同一棵树的后序遍历，重构并返回二叉树。
     * 如果存在多个答案，您可以返回其中 任何 一个。
     */
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(preor: &[i32], postor: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preor.len() == 0 {
                return None;
            }
            let mut root = TreeNode::new(preor[0]);
            if let Some(left_root) = preor.get(1) {
                let left_root_idx = (0..postor.len() - 1)
                    .find(|idx| postor[*idx] == *left_root)
                    .unwrap();
                root.left = build(&preor[1..=left_root_idx + 1], &postor[0..=left_root_idx]);
                if let Some(right_root) = preor.get(left_root_idx + 2) {
                    root.right = build(
                        &preor[left_root_idx + 2..],
                        &postor[left_root_idx + 1..postor.len() - 1],
                    );
                }
            }
            Some(Rc::new(RefCell::new(root)))
        }
        build(&preorder[..], &postorder[..])
    }

    /**
     * 2583. 二叉树中的第 K 大层和
     * 给你一棵二叉树的根节点 root 和一个正整数 k 。
     * 树中的 层和 是指 同一层 上节点值的总和。
     * 返回树中第 k 大的层和（不一定不同）。如果树少于 k 层，则返回 -1 。
     * 注意，如果两个节点与根节点的距离相同，则认为它们在同一层。
     */
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        use std::cmp::Reverse;
        use std::collections::VecDeque;
        let mut level_sum = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            let mut curr_sum = 0;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                curr_sum += node.borrow().val as i64;
                if let Some(left) = node.borrow_mut().left.take() {
                    queue.push_back(left);
                };
                if let Some(right) = node.borrow_mut().right.take() {
                    queue.push_back(right);
                };
            }
            level_sum.push(Reverse(curr_sum));
        }
        level_sum.sort_unstable();
        if let Some(ret) = level_sum.get(k as usize - 1) {
            ret.0
        } else {
            -1
        }
    }

    /**
     * 2476. 二叉搜索树最近节点查询
     * 给你一个 二叉搜索树 的根节点 root ，和一个由正整数组成、长度为 n 的数组 queries 。
     * 请你找出一个长度为 n 的 二维 答案数组 answer ，其中 answer[i] = [mini, maxi] ：
     *     mini 是树中小于等于 queries[i] 的 最大值 。如果不存在这样的值，则使用 -1 代替。
     *     maxi 是树中大于等于 queries[i] 的 最小值 。如果不存在这样的值，则使用 -1 代替。
     * 返回数组 answer 。
     */
    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        fn inorder(node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
            if let Some(n) = node {
                inorder(n.borrow_mut().left.take(), arr);
                arr.push(n.borrow().val);
                inorder(n.borrow_mut().right.take(), arr);
            }
        }
        let mut arr = Vec::new();
        inorder(root, &mut arr);
        let mut ret = Vec::new();
        for q in queries {
            let idx = arr.partition_point(|&v| v < q);
            if idx == arr.len() {
                ret.push(vec![arr[idx - 1], -1]);
            } else {
                if arr[idx] == q {
                    ret.push(vec![q, q]);
                    continue;
                }
                if idx == 0 {
                    ret.push(vec![-1, arr[idx]]);
                } else {
                    ret.push(vec![arr[idx - 1], arr[idx]]);
                }
            }
        }
        ret
    }

    /**
     * 1976. 到达目的地的方案数
     * 你在一个城市里，城市由 n 个路口组成，路口编号为 0 到 n - 1 ，
     * 某些路口之间有 双向 道路。输入保证你可以从任意路口出发到达其他任意路口，且任意两个路口之间最多有一条路。
     * 给你一个整数 n 和二维整数数组 roads ，其中 roads[i] = [ui, vi, timei] 表示在路口 ui 和 vi 之间有一条需要花费 timei 时间才能通过的道路。
     * 你想知道花费 最少时间 从路口 0 出发到达路口 n - 1 的方案数。
     * 请返回花费 最少时间 到达目的地的 路径数目 。由于答案可能很大，将结果对 109 + 7 取余 后返回。
     */
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut adj = vec![vec![]; n as usize];
        for r in roads {
            adj[r[0] as usize].push((r[1], r[2]));
            adj[r[1] as usize].push((r[0], r[2]));
        }
        let mut distance = vec![i64::MAX; n as usize];
        let mut visited = vec![false; n as usize];
        let mut min = BinaryHeap::new();
        let mut ways = vec![0; n as usize];
        ways[0] = 1;
        distance[0] = 0;
        min.push((Reverse(0), 0));
        while let Some((_, curr)) = min.pop() {
            let curr = curr as usize;
            if visited[curr] {
                continue;
            }
            visited[curr] = true;
            for &adj_nodes in &adj[curr] {
                let node = adj_nodes.0 as usize;
                let dis = adj_nodes.1 as i64;
                if distance[curr] + dis < distance[node] {
                    distance[node] = distance[curr] + dis;
                    min.push((Reverse(distance[node]), adj_nodes.0));
                    ways[node] = ways[curr];
                } else if distance[curr] + dis == distance[node] {
                    ways[node] = (ways[curr] + ways[node]) % 1_000_000_007;
                }
            }
        }
        ways[n as usize - 1]
    }

    /**
     * 2575. 找出字符串的可整除数组
     * 给你一个下标从 0 开始的字符串 word ，长度为 n ，由从 0 到 9 的数字组成。另给你一个正整数 m 。
     * word 的 可整除数组 div  是一个长度为 n 的整数数组，并满足：
     *     如果 word[0,...,i] 所表示的 数值 能被 m 整除，div[i] = 1
     *     否则，div[i] = 0
     * 返回 word 的可整除数组。
     */
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let mut remainder = 0;
        word.chars()
            .map(|n| {
                remainder = (remainder * 10 + (n.to_digit(10).unwrap() as i64)) % m as i64;
                match remainder {
                    0 => 1,
                    _ => 0,
                }
            })
            .collect()
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
