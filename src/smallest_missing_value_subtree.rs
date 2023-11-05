use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    /**
     *有一棵根节点为 0 的 家族树 ，总共包含 n 个节点，节点编号为 0 到 n - 1 。
     *给你一个下标从 0 开始的整数数组 parents ，其中 parents[i] 是节点 i 的父节点。由于节点 0 是 根 ，所以 parents[0] == -1 。
     *总共有 10^5 个基因值，每个基因值都用 闭区间 [1, 10^5] 中的一个整数表示。
     *给你一个下标从 0 开始的整数数组 nums ，其中 nums[i] 是节点 i 的基因值，且基因值 互不相同 。
     *请你返回一个数组 ans ，长度为 n ，其中 ans[i] 是以节点 i 为根的子树内 缺失 的 最小 基因值。
     *节点 x 为根的 子树 包含节点 x 和它所有的 后代 节点。
     */
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let size = parents.len();
        let mut result = vec![1; size];
        // 不存在基因值为1的点，说明所有的节点缺失的最小基因都为1，直接返回答案
        let mut node = match nums.iter().position(|&x| x == 1) {
            Some(i) => i as i32,
            None => return result,
        };

        // 方便遍历树
        let mut tree = vec![vec![]; size];
        for i in 1..size {
            tree[parents[i] as usize].push(i as i32);
        }

        //由于最小缺失基因值不为1的节点一定在从基因值为1的节点到根节点的一条路上，所以可以把所有基因值保存在一个hash表里面判断
        let mut all_child_nums: HashSet<i32> = HashSet::new();
        let mut stack: Vec<i32> = Vec::new();
        let mut smallest = 2;
        while node >= 0 {
            stack.clear();
            stack.push(node);
            while !stack.is_empty() {
                let x = stack.pop().unwrap();
                all_child_nums.insert(nums[x as usize]);
                stack.append(&mut tree[x as usize]);
            }
            while all_child_nums.contains(&smallest) {
                smallest += 1;
            }
            result[node as usize] = smallest;
            node = parents[node as usize];
        }
        result
    }
}
