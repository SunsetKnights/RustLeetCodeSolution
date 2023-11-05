use std::collections::VecDeque;

pub struct Solution {}
/**
 * 2127. 参加会议的最多员工数
 *一个公司准备组织一场会议，邀请名单上有 n 位员工。公司准备了一张 圆形 的桌子，可以坐下 任意数目 的员工。
 *员工编号为 0 到 n - 1 。每位员工都有一位 喜欢 的员工，每位员工 当且仅当 他被安排在喜欢员工的旁边，他才会参加会议。每位员工喜欢的员工不会是他自己。
 *给你一个下标从 0 开始的整数数组 favorite ，其中 favorite[i] 表示第 i 位员工喜欢的员工。请你返回参加会议的 最多员工数目 。
 */
impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        //超时，重写
        // //1、寻找图中存在的所有的环
        // let mut result;
        // //从任意一节点出发，找到已经遍历过的节点时，找到了一个环
        // //若从某个节点出发找到了一个环，那么从这个节点出发不可能找到其他的环
        // //至少有一个环，但可能不止一个环
        // let mut all_ring: Vec<Vec<usize>> = Vec::new();
        // let mut readed_node: Vec<i32> = vec![0; favorite.len()];
        // let mut temp: Vec<usize> = Vec::new();
        // let mut node: i32;
        // let mut start: usize;
        // 'outer: loop {
        //     start = match readed_node.iter().position(|x| *x == 0) {
        //         Some(i) => i,
        //         None => break,
        //     };
        //     temp.push(start);
        //     readed_node[start] = 1;
        //     node = favorite[start];
        //     while !temp.contains(&(node as usize)) {
        //         temp.push(node as usize);
        //         if readed_node[node as usize] == 0 {
        //             readed_node[node as usize] = 1;
        //         } else {
        //             //重复的环
        //             temp.clear();
        //             continue 'outer;
        //         }
        //         node = favorite[node as usize];
        //     }
        //     //找到了一个新的环
        //     start = temp.iter().position(|x| *x == node as usize).unwrap();
        //     let mut ring: Vec<usize> = Vec::new();
        //     ring.extend_from_slice(&temp[start..]);
        //     all_ring.push(ring);
        //     temp.clear();
        // }
        // //2、求出所有长度为2的环的最大节点数
        // fn max_deep(target: usize, exception: usize, favorite_rev: &Vec<Vec<usize>>) -> i32 {
        //     let mut max = 0;
        //     let mut curr_deep = 0;
        //     let mut curr_node = target;
        //     let mut stack: Vec<(usize, i32)> = Vec::new();
        //     stack.push((curr_node as usize, curr_deep));
        //     while stack.len() != 0 {
        //         let t = stack.pop().unwrap();
        //         curr_node = t.0;
        //         curr_deep = t.1;
        //         if curr_deep > max {
        //             max = curr_deep;
        //         }
        //         curr_deep += 1;
        //         for i in favorite_rev[curr_node].iter() {
        //             if *i == exception {
        //                 continue;
        //             }
        //             stack.push((*i, curr_deep));
        //         }
        //     }
        //     max
        // }
        // let mut favorite_rev = vec![vec![]; favorite.len()];
        // for (i, v) in favorite.iter().enumerate() {
        //     favorite_rev[*v as usize].push(i);
        // }
        // let mut maximumes: Vec<i32> = Vec::new();
        // let mut maximum_2 = 0;
        // for i in all_ring {
        //     if i.len() == 2 {
        //         maximum_2 +=
        //             max_deep(i[0], i[1], &favorite_rev) + max_deep(i[1], i[0], &favorite_rev) + 2;
        //     } else {
        //         maximumes.push(i.len() as i32);
        //     }
        // }
        // result = maximum_2;
        // for i in maximumes {
        //     if i > result {
        //         result = i;
        //     }
        // }
        // result

        //将环和树分开，拓扑排序，入度为1的节点分离出来，为环
        let size = favorite.len();
        let mut inner_degree = vec![0; size];
        for i in favorite.iter() {
            inner_degree[*i as usize] += 1;
        }
        //拓扑排序时，可以求出每个以环内节点为根节点的树的最大深度
        let mut inner_degree_0: VecDeque<usize> = VecDeque::new();
        //找到所有入度为0的节点
        for (i, v) in inner_degree.iter().enumerate() {
            if *v == 0 {
                inner_degree_0.push_back(i);
            }
        }
        let mut max_deep = vec![1; size];
        //剪枝，除环以外的所有节点的入度都将为0
        while let Some(x) = inner_degree_0.pop_front() {
            let out_node = favorite[x] as usize;
            max_deep[out_node] = max_deep[x] + 1;
            inner_degree[out_node] -= 1;
            if inner_degree[out_node] == 0 {
                inner_degree_0.push_back(out_node);
            }
        }
        //寻找所有环，比较
        let mut max_ring: i32 = 0; //环中不止两个节点时，取最大
        let mut max_chain: i32 = 0; //环中只有两个节点时，求和
        let mut curr_ring;
        let mut curr_node;
        while let Some(start) = inner_degree.iter().position(|x| *x == 1) {
            curr_ring = 1;
            inner_degree[start] = 0;
            curr_node = favorite[start];
            while curr_node != start as i32 {
                curr_ring += 1;
                inner_degree[curr_node as usize] = 0;
                curr_node = favorite[curr_node as usize];
            }
            if curr_ring == 2 {
                max_chain += max_deep[start] + max_deep[favorite[start] as usize];
            } else {
                max_ring = if max_ring > curr_ring {
                    max_ring
                } else {
                    curr_ring
                };
            }
        }
        if max_chain > max_ring {
            max_chain
        } else {
            max_ring
        }
    }
}
