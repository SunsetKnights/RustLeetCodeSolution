pub struct Solution;
/**
 * 2846. 边权重均等查询
 * 现有一棵由 n 个节点组成的无向树，节点按从 0 到 n - 1 编号。
 * 给你一个整数 n 和一个长度为 n - 1 的二维整数数组 edges ，
 * 其中 edges[i] = [ui, vi, wi] 表示树中存在一条位于节点 ui 和节点 vi 之间、权重为 wi 的边。
 *
 * 另给你一个长度为 m 的二维整数数组 queries ，其中 queries[i] = [ai, bi] 。
 * 对于每条查询，请你找出使从 ai 到 bi 路径上每条边的权重相等所需的 最小操作次数 。
 * 在一次操作中，你可以选择树上的任意一条边，并将其权重更改为任意值。
 *
 * 注意：
 *     查询之间 相互独立 的，这意味着每条新的查询时，树都会回到 初始状态 。
 *     从 ai 到 bi的路径是一个由 不同 节点组成的序列，从节点 ai 开始，到节点 bi 结束，且序列中相邻的两个节点在树中共享一条边。
 * 返回一个长度为 m 的数组 answer ，其中 answer[i] 是第 i 条查询的答案。
 */
impl Solution {
    pub fn min_operations_queries(
        n: i32,
        edges: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut graphics = vec![Vec::new(); n];
        for edge in edges {
            graphics[edge[0] as usize].push((edge[1], edge[2] - 1));
            graphics[edge[1] as usize].push((edge[0], edge[2] - 1));
        }
        let m = (64 - n.leading_zeros()) as usize;
        let mut parents = vec![vec![-1; m]; n];
        let mut w_counter = vec![vec![[0; 26]; m]; n];
        let mut deep = vec![0; n];
        fn dfs(
            node: i32,
            father: i32,
            graphics: &Vec<Vec<(i32, i32)>>,
            parents: &mut Vec<Vec<i32>>,
            w_counter: &mut Vec<Vec<[i32; 26]>>,
            deep: &mut Vec<i32>,
        ) {
            parents[node as usize][0] = father;
            for (c, w) in graphics[node as usize].iter() {
                if *c != father {
                    w_counter[*c as usize][0][*w as usize] = 1;
                    deep[*c as usize] = deep[node as usize] + 1;
                    dfs(*c, node as i32, graphics, parents, w_counter, deep);
                }
            }
        }
        dfs(0, -1, &graphics, &mut parents, &mut w_counter, &mut deep);
        for i in 0..(m - 1) as usize {
            for j in 0..n as usize {
                let p = parents[j][i];
                if p != -1 {
                    let grand_p = parents[p as usize][i];
                    parents[j][i + 1] = grand_p;
                    for k in 0..26 {
                        w_counter[j][i + 1][k] = w_counter[j][i][k] + w_counter[p as usize][i][k];
                    }
                }
            }
        }
        let mut ret = Vec::with_capacity(queries.len());
        for query in queries {
            let mut x = query[0] as usize;
            let mut y = query[1] as usize;
            let mut path_len = deep[x] + deep[y];
            if deep[x] > deep[y] {
                x ^= y;
                y ^= x;
                x ^= y;
            }
            // 减少y深度
            let mut temp_w_counter = [0; 26];
            let mut k = deep[y] - deep[x];
            while k != 0 {
                let idx = k.trailing_zeros() as usize;
                let parent = parents[y][idx];
                for i in 0..26 {
                    temp_w_counter[i] += w_counter[y][idx][i];
                }
                y = parent as usize;
                k &= k - 1;
            }
            if y != x {
                for i in (0..m as usize).rev() {
                    let x_parent = parents[x][i];
                    let y_parent = parents[y][i];
                    if x_parent != y_parent {
                        for j in 0..26 {
                            temp_w_counter[j] += w_counter[x][i][j] + w_counter[y][i][j];
                        }
                        x = x_parent as usize;
                        y = y_parent as usize;
                    }
                }
                for j in 0..26 {
                    temp_w_counter[j] += w_counter[x][0][j] + w_counter[y][0][j];
                }
                x = parents[x][0] as usize;
            }
            path_len -= deep[x] * 2;
            let mut max = 0;
            for c in temp_w_counter {
                max = max.max(c);
            }
            ret.push(path_len - max);
        }
        ret
    }
}
