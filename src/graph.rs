use std::{cmp::Reverse, collections::BinaryHeap};

/**
 * 2642. 设计可以求最短路径的图类
 * 给你一个有 n 个节点的 有向带权 图，节点编号为 0 到 n - 1
 * 。图中的初始边用数组 edges 表示，其中 edges[i] = [fromi, toi, edgeCosti] 表示从 fromi 到 toi 有一条代价为 edgeCosti 的边。
 * 请你实现一个 Graph 类：
 *     Graph(int n, int[][] edges) 初始化图有 n 个节点，并输入初始边。
 *     addEdge(int[] edge) 向边集中添加一条边，其中 edge = [from, to, edgeCost] 。数据保证添加这条边之前对应的两个节点之间没有有向边。
 *     int shortestPath(int node1, int node2) 返回从节点 node1 到 node2 的路径 最小 代价。如果路径不存在，返回 -1 。一条路径的代价是路径中所有边代价之和。
 */
pub struct Graph {
    g: Vec<Vec<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {
    pub fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut g = vec![vec![]; n as usize];
        for edge in edges {
            g[edge[0] as usize].push((edge[1], edge[2]));
        }
        Self { g }
    }

    pub fn add_edge(&mut self, edge: Vec<i32>) {
        self.g[edge[0] as usize].push((edge[1], edge[2]));
    }

    pub fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        // 按距离建立小根堆
        let mut queue: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
        let mut distance: Vec<i32> = vec![i32::MAX; self.g.len()];
        let mut visited: Vec<bool> = vec![false; self.g.len()];
        distance[node1 as usize] = 0;
        queue.push((Reverse(0), node1));
        while let Some(d) = queue.pop() {
            // 找到一个端点已经访问过，而令一个端点没有被访问过的最小的边
            let node = d.1 as usize;
            if visited[node] {
                continue;
            }
            // 把这条边的令一个端点标记为已访问
            visited[node] = true;
            // 遍历最后被访问的节点的所有边，看距离是否需要更新
            for &e in &self.g[node] {
                let next_node = e.0 as usize;
                let next_distance = e.1;
                if distance[next_node] > distance[node] + next_distance {
                    distance[next_node] = distance[node] + next_distance;
                    queue.push((Reverse(distance[next_node]), next_node as i32));
                }
            }
        }
        match distance[node2 as usize] {
            i32::MAX => -1,
            dis => dis,
        }
    }
}
