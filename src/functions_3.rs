struct Solution;
#[allow(unused)]
impl Solution {
    /**
     * 2580. 统计将重叠区间合并成组的方案数
     * 给你一个二维整数数组 ranges ，其中 ranges[i] = [starti, endi] 表示 starti 到 endi 之间（包括二者）的所有整数都包含在第 i 个区间中。
     * 你需要将 ranges 分成 两个 组（可以为空），满足：
     *     每个区间只属于一个组。
     *     两个有 交集 的区间必须在 同一个 组内。
     * 如果两个区间有至少 一个 公共整数，那么这两个区间是 有交集 的。
     *     比方说，区间 [1, 3] 和 [2, 5] 有交集，因为 2 和 3 在两个区间中都被包含。
     * 请你返回将 ranges 划分成两个组的 总方案数 。由于答案可能很大，将它对 109 + 7 取余 后返回。
     */
    pub fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
        ranges.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut res = 1i64;
        let mut i = 0;
        while i < ranges.len() {
            let mut right = ranges[i][1];
            let mut j = i + 1;
            while j < ranges.len() && right >= ranges[j][0] {
                right = right.max(ranges[j][1]);
                j += 1;
            }
            res = (res * 2) % 1_000_000_007;
            i = j;
        }
        res as i32
    }

    /**
     * 1997. 访问完所有房间的第一天
     * 你需要访问 n 个房间，房间从 0 到 n - 1 编号。同时，每一天都有一个日期编号，从 0 开始，依天数递增。你每天都会访问一个房间。
     * 最开始的第 0 天，你访问 0 号房间。给你一个长度为 n 且 下标从 0 开始 的数组 nextVisit 。在接下来的几天中，你访问房间的 次序 将根据下面的 规则 决定：
     *     假设某一天，你访问 i 号房间。
     *     如果算上本次访问，访问 i 号房间的次数为 奇数 ，那么 第二天 需要访问 nextVisit[i] 所指定的房间，其中 0 <= nextVisit[i] <= i 。
     *     如果算上本次访问，访问 i 号房间的次数为 偶数 ，那么 第二天 需要访问 (i + 1) mod n 号房间。
     * 请返回你访问完所有房间的第一天的日期编号。题目数据保证总是存在这样的一天。由于答案可能很大，返回对 109 + 7 取余后的结果。
     *
     * n == nextVisit.length
     * 2 <= n <= 105
     * 0 <= nextVisit[i] <= i
     */
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        let mut prefix = vec![0; next_visit.len()];
        // 第一次到达0+1号房间的天数是2
        prefix[0] = 2;
        for i in 1..next_visit.len() {
            prefix[i] = prefix[i - 1] + 2;
            if next_visit[i] != 0 {
                prefix[i] = (prefix[i] - prefix[(next_visit[i] - 1) as usize] + 1_000_000_007)
                    % 1_000_000_007;
            }
            prefix[i] = (prefix[i] + prefix[i - 1]) % 1_000_000_007;
        }
        prefix[next_visit.len() - 2]
    }

    /**
     * 2952. 需要添加的硬币的最小数量
     * 给你一个下标从 0 开始的整数数组 coins，表示可用的硬币的面值，以及一个整数 target 。
     * 如果存在某个 coins 的子序列总和为 x，那么整数 x 就是一个 可取得的金额 。
     * 返回需要添加到数组中的 任意面值 硬币的 最小数量 ，使范围 [1, target] 内的每个整数都属于 可取得的金额 。
     * 数组的 子序列 是通过删除原始数组的一些（可能不删除）元素而形成的新的 非空 数组，删除过程不会改变剩余元素的相对位置。
     */
    pub fn minimum_added_coins(mut coins: Vec<i32>, target: i32) -> i32 {
        coins.sort_unstable();
        let (mut added, mut curr_complete, mut i) = (0, 1, 0usize);
        while curr_complete <= target {
            if i < coins.len() && coins[i] <= curr_complete {
                curr_complete += coins[i];
                i += 1;
            } else {
                // add num curr_complete
                curr_complete += curr_complete;
                added += 1;
            }
        }
        added
    }

    /**
     * 331. 验证二叉树的前序序列化
     * 序列化二叉树的一种方法是使用 前序遍历 。当我们遇到一个非空节点时，我们可以记录下这个节点的值。如果它是一个空节点，我们可以使用一个标记值记录，例如 #。
     * 给定一串以逗号分隔的序列，验证它是否是正确的二叉树的前序序列化。编写一个在不重构树的条件下的可行算法。
     * 保证 每个以逗号分隔的字符或为一个整数或为一个表示 null 指针的 '#' 。
     * 你可以认为输入格式总是有效的
     *     例如它永远不会包含两个连续的逗号，比如 "1,,3" 。
     * 注意：不允许重建树。
     */
    pub fn is_valid_serialization(preorder: String) -> bool {
        let chars: Vec<char> = preorder
            .split(',')
            .map(|s| s.chars().next().unwrap())
            .collect();
        let mut stack = Vec::new();
        for c in chars {
            stack.push(c);
            let mut len = stack.len();
            let mut top = c;
            while top == '#'
                && len >= 3
                && stack[len - 1] == '#'
                && stack[len - 2] == '#'
                && stack[len - 3] != '#'
            {
                stack.pop();
                stack.pop();
                stack.pop();
                stack.push('#');
                top = '#';
                len = stack.len();
            }
        }
        stack.len() == 1 && stack[0] == '#'
    }
    /**
     * 2192. 有向无环图中一个节点的所有祖先
     * 给你一个正整数 n ，它表示一个 有向无环图 中节点的数目，节点编号为 0 到 n - 1 （包括两者）。
     * 给你一个二维整数数组 edges ，其中 edges[i] = [fromi, toi] 表示图中一条从 fromi 到 toi 的单向边。
     * 请你返回一个数组 answer，其中 answer[i]是第 i 个节点的所有 祖先 ，这些祖先节点 升序 排序。
     * 如果 u 通过一系列边，能够到达 v ，那么我们称节点 u 是节点 v 的 祖先 节点。
     */
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut g = vec![vec![]; n as usize];
        for e in edges {
            g[e[1] as usize].push(e[0]);
        }
        fn dfs(g: &Vec<Vec<i32>>, curr: i32, visited: &mut Vec<bool>) {
            for &next in &g[curr as usize] {
                if !visited[next as usize] {
                    visited[next as usize] = true;
                    dfs(g, next, visited);
                }
            }
        }
        let mut res = vec![vec![]; n as usize];
        for i in 0..n {
            let mut visited = vec![false; n as usize];
            dfs(&g, i, &mut visited);
            for j in 0..n {
                if visited[j as usize] {
                    res[i as usize].push(j);
                }
            }
        }
        res
    }
}
