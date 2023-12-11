use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;
/**
 * 1631. 最小体力消耗路径
 * 你准备参加一场远足活动。给你一个二维 rows x columns 的地图 heights ，其中 heights[row][col] 表示格子 (row, col) 的高度。
 * 一开始你在最左上角的格子 (0, 0) ，且你希望去最右下角的格子 (rows-1, columns-1) （注意下标从 0 开始编号）。
 * 你每次可以往 上，下，左，右 四个方向之一移动，你想要找到耗费 体力 最小的一条路径。
 * 一条路径耗费的 体力值 是路径上相邻格子之间 高度差绝对值 的 最大值 决定的。
 * 请你返回从左上角走到右下角的最小 体力消耗值 。
 */
impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let row = heights.len();
        let col = heights[0].len();
        let n = row * col;
        let mut edges: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n];
        for (i, r) in heights.iter().enumerate() {
            for (j, val) in r.iter().enumerate() {
                if i > 0 {
                    edges[i * col + j].push(((i - 1) * col + j, (val - heights[i - 1][j]).abs()));
                }
                if i < row - 1 {
                    edges[i * col + j].push(((i + 1) * col + j, (val - heights[i + 1][j]).abs()));
                }
                if j > 0 {
                    edges[i * col + j].push((i * col + j - 1, (val - heights[i][j - 1]).abs()));
                }
                if j < col - 1 {
                    edges[i * col + j].push((i * col + j + 1, (val - heights[i][j + 1]).abs()));
                }
            }
        }

        let dijkstra = |x: usize| -> Vec<i32> {
            // 按距离建立小根堆
            let mut queue: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
            let mut distance: Vec<i32> = (0..n).map(|_| i32::MAX).collect();
            let mut visited: Vec<bool> = (0..n).map(|_| false).collect();
            distance[x] = 0;
            queue.push((Reverse(0), x));
            while let Some(d) = queue.pop() {
                // 找到一个端点已经访问过，而令一个端点没有被访问过的最小的边
                let node = d.1;
                if visited[node] {
                    continue;
                }
                // 把这条边的令一个端点标记为已访问
                visited[node] = true;
                // 遍历最后被访问的节点的所有边，看距离是否需要更新
                for e in &edges[node] {
                    let next_node = e.0;
                    let next_distance = e.1;
                    if distance[next_node] > distance[node].max(next_distance) {
                        distance[next_node] = distance[node].max(next_distance);
                        queue.push((Reverse(distance[next_node]), next_node));
                    }
                }
            }
            distance
        };

        let ret = dijkstra(0);
        ret[n - 1]
    }
}
