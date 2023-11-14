use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}
/**
 * 1334. 阈值距离内邻居最少的城市
 * 有 n 个城市，按从 0 到 n-1 编号。给你一个边数组 edges，其中 edges[i] = [fromi, toi, weighti] 代表 fromi 和 toi 两个城市之间的双向加权边，距离阈值是一个整数 distanceThreshold。
 * 返回能通过某些路径到达其他城市数目最少、且路径距离 最大 为 distanceThreshold 的城市。如果有多个这样的城市，则返回编号最大的城市。
 * 注意，连接城市 i 和 j 的路径的距离等于沿该路径的所有边的权重之和。
 */
impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let mut eedges: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize];
        for e in edges {
            eedges[e[0] as usize].push((e[1], e[2]));
            eedges[e[1] as usize].push((e[0], e[2]));
        }

        let dijkstra = |x: usize| -> Vec<i32> {
            // 按距离建立小根堆
            let mut queue: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
            let mut distance: Vec<i32> = (0..n as usize).map(|_| i32::MAX).collect();
            let mut visited: Vec<bool> = (0..n as usize).map(|_| false).collect();
            distance[x as usize] = 0;
            queue.push((Reverse(0), x));
            while let Some(d) = queue.pop() {
                // 找到一个端点已经访问过，而令一个端点没有被访问过的最小的边
                let node = d.1 as usize;
                if visited[node] {
                    continue;
                }
                // 把这条边的令一个端点标记为已访问
                visited[node] = true;
                // 遍历最后被访问的节点的所有边，看距离是否需要更新
                for e in &eedges[node] {
                    let next_node = e.0 as usize;
                    let next_distance = e.1;
                    if distance[next_node] > distance[node] + next_distance {
                        distance[next_node] = distance[node] + next_distance;
                        queue.push((Reverse(distance[next_node]), next_node));
                    }
                }
            }
            distance
        };

        let mut ret = (0, i32::MAX);
        for i in 0..n as usize {
            let mut quantity = 0;
            let d = dijkstra(i);
            for i in d {
                if i <= distance_threshold {
                    quantity += 1;
                }
            }
            if quantity < ret.1 {
                ret = (i, quantity);
            } else if quantity == ret.1 && i > ret.0 {
                ret.0 = i;
            }
        }
        ret.0 as i32
    }
}
