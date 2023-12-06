pub struct Solution;
/**
 * 2646. 最小化旅行的价格总和
 * 现有一棵无向、无根的树，树中有 n 个节点，按从 0 到 n - 1 编号。给你一个整数 n 和一个长度为 n - 1 的二维整数数组 edges ，其中 edges[i] = [ai, bi] 表示树中节点 ai 和 bi 之间存在一条边。
 * 每个节点都关联一个价格。给你一个整数数组 price ，其中 price[i] 是第 i 个节点的价格。
 * 给定路径的 价格总和 是该路径上所有节点的价格之和。
 * 另给你一个二维整数数组 trips ，其中 trips[i] = [starti, endi] 表示您从节点 starti 开始第 i 次旅行，并通过任何你喜欢的路径前往节点 endi 。
 * 在执行第一次旅行之前，你可以选择一些 非相邻节点 并将价格减半。
 * 返回执行所有旅行的最小价格总和。
 */
impl Solution {
    pub fn minimum_total_price(
        n: i32,
        edges: Vec<Vec<i32>>,
        price: Vec<i32>,
        trips: Vec<Vec<i32>>,
    ) -> i32 {
        let n = n as usize;
        let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); n];
        for i in edges {
            let s = i[0] as usize;
            let e = i[1] as usize;
            adj_list[s].push(e);
            adj_list[e].push(s);
        }
        // DFS统计每个节点经过的次数
        fn dfs_counter(
            start: usize,
            end: usize,
            from: usize,
            counter: &mut Vec<i32>,
            adj_list: &Vec<Vec<usize>>,
        ) -> bool {
            if start == end {
                return true;
            }
            for &i in &adj_list[start] {
                if i != from && dfs_counter(i, end, start, counter, adj_list) {
                    counter[i] += 1;
                    return true;
                }
            }
            return false;
        }
        let mut counter = vec![0; n];
        for trip in trips {
            let start = trip[0] as usize;
            let end = trip[1] as usize;
            counter[start] += 1;
            dfs_counter(start, end, start, &mut counter, &adj_list);
        }
        // 统计节点经过次数和节点价格乘积和，把一些不相邻的节点价格/2，使和最小
        fn dfs_price(
            curr: usize,
            from: usize,
            counter: &Vec<i32>,
            price: &Vec<i32>,
            adj_list: &Vec<Vec<usize>>,
        ) -> (i32, i32) {
            let mut curr_not_half = counter[curr] * price[curr];
            let mut curr_half = curr_not_half / 2;
            for &i in &adj_list[curr] {
                if i != from {
                    let (not_half, half) = dfs_price(i, curr, counter, price, adj_list);
                    curr_half += not_half;
                    curr_not_half = (curr_not_half + not_half).min(curr_not_half + half);
                }
            }
            (curr_not_half, curr_half)
        }
        let (not_half, half) = dfs_price(0, 0, &counter, &price, &adj_list);
        not_half.min(half)
    }
}
