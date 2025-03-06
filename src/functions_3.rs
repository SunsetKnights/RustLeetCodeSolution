pub struct Solution;
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
    /**
     * 1702. 修改后的最大二进制字符串
     * 给你一个二进制字符串 binary ，它仅有 0 或者 1 组成。你可以使用下面的操作任意次对它进行修改：
     *     操作 1 ：如果二进制串包含子字符串 "00" ，你可以用 "10" 将其替换。
     *         比方说， "00010" -> "10010"
     *     操作 2 ：如果二进制串包含子字符串 "10" ，你可以用 "01" 将其替换。
     *         比方说， "00010" -> "00001"
     * 请你返回执行上述操作任意次以后能得到的 最大二进制字符串 。
     * 如果二进制字符串 x 对应的十进制数字大于二进制字符串 y 对应的十进制数字，那么我们称二进制字符串 x 大于二进制字符串 y 。
     */
    pub fn maximum_binary_string(binary: String) -> String {
        if let Some(i) = binary.find('0') {
            let one_count = binary[i..].chars().filter(|&c| c == '1').count();
            return "1".repeat(binary.len() - 1 - one_count) + "0" + &"1".repeat(one_count);
        }
        binary
    }
    /**
     * 1766. 互质树
     * 给你一个 n 个节点的树（也就是一个无环连通无向图），节点编号从 0 到 n - 1 ，且恰好有 n - 1 条边，每个节点有一个值。树的 根节点 为 0 号点。
     * 给你一个整数数组 nums 和一个二维数组 edges 来表示这棵树。nums[i] 表示第 i 个点的值，edges[j] = [uj, vj] 表示节点 uj 和节点 vj 在树中有一条边。
     * 当 gcd(x, y) == 1 ，我们称两个数 x 和 y 是 互质的 ，其中 gcd(x, y) 是 x 和 y 的 最大公约数 。
     * 从节点 i 到 根 最短路径上的点都是节点 i 的祖先节点。一个节点 不是 它自己的祖先节点。
     * 请你返回一个大小为 n 的数组 ans ，其中 ans[i]是离节点 i 最近的祖先节点且满足 nums[i] 和 nums[ans[i]] 是 互质的 ，
     * 如果不存在这样的祖先节点，ans[i] 为 -1 。
     * 1 <= nums[i] <= 50
     */
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let coprimes = vec![
            vec![],
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
                45, 46, 47, 48, 49, 50,
            ],
            vec![
                1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43,
                45, 47, 49,
            ],
            vec![
                1, 2, 4, 5, 7, 8, 10, 11, 13, 14, 16, 17, 19, 20, 22, 23, 25, 26, 28, 29, 31, 32,
                34, 35, 37, 38, 40, 41, 43, 44, 46, 47, 49, 50,
            ],
            vec![
                1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43,
                45, 47, 49,
            ],
            vec![
                1, 2, 3, 4, 6, 7, 8, 9, 11, 12, 13, 14, 16, 17, 18, 19, 21, 22, 23, 24, 26, 27, 28,
                29, 31, 32, 33, 34, 36, 37, 38, 39, 41, 42, 43, 44, 46, 47, 48, 49,
            ],
            vec![
                1, 5, 7, 11, 13, 17, 19, 23, 25, 29, 31, 35, 37, 41, 43, 47, 49,
            ],
            vec![
                1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 15, 16, 17, 18, 19, 20, 22, 23, 24, 25, 26,
                27, 29, 30, 31, 32, 33, 34, 36, 37, 38, 39, 40, 41, 43, 44, 45, 46, 47, 48, 50,
            ],
            vec![
                1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43,
                45, 47, 49,
            ],
            vec![
                1, 2, 4, 5, 7, 8, 10, 11, 13, 14, 16, 17, 19, 20, 22, 23, 25, 26, 28, 29, 31, 32,
                34, 35, 37, 38, 40, 41, 43, 44, 46, 47, 49, 50,
            ],
            vec![
                1, 3, 7, 9, 11, 13, 17, 19, 21, 23, 27, 29, 31, 33, 37, 39, 41, 43, 47, 49,
            ],
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 23, 24, 25,
                26, 27, 28, 29, 30, 31, 32, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 45, 46, 47, 48,
                49, 50,
            ],
            vec![
                1, 5, 7, 11, 13, 17, 19, 23, 25, 29, 31, 35, 37, 41, 43, 47, 49,
            ],
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                25, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 40, 41, 42, 43, 44, 45, 46, 47,
                48, 49, 50,
            ],
            vec![
                1, 3, 5, 9, 11, 13, 15, 17, 19, 23, 25, 27, 29, 31, 33, 37, 39, 41, 43, 45, 47,
            ],
            vec![
                1, 2, 4, 7, 8, 11, 13, 14, 16, 17, 19, 22, 23, 26, 28, 29, 31, 32, 34, 37, 38, 41,
                43, 44, 46, 47, 49,
            ],
            vec![
                1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43,
                45, 47, 49,
            ],
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21, 22, 23, 24,
                25, 26, 27, 28, 29, 30, 31, 32, 33, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
                47, 48, 49, 50,
            ],
            vec![
                1, 5, 7, 11, 13, 17, 19, 23, 25, 29, 31, 35, 37, 41, 43, 47, 49,
            ],
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22, 23, 24,
                25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 39, 40, 41, 42, 43, 44, 45, 46,
                47, 48, 49, 50,
            ],
            vec![
                1, 3, 7, 9, 11, 13, 17, 19, 21, 23, 27, 29, 31, 33, 37, 39, 41, 43, 47, 49,
            ],
            vec![
                1, 2, 4, 5, 8, 10, 11, 13, 16, 17, 19, 20, 22, 23, 25, 26, 29, 31, 32, 34, 37, 38,
                40, 41, 43, 44, 46, 47, 50,
            ],
            vec![
                1, 3, 5, 7, 9, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 35, 37, 39, 41, 43, 45, 47,
                49,
            ],
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 24,
                25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
                47, 48, 49, 50,
            ],
            vec![
                1, 5, 7, 11, 13, 17, 19, 23, 25, 29, 31, 35, 37, 41, 43, 47, 49,
            ],
            vec![
                1, 2, 3, 4, 6, 7, 8, 9, 11, 12, 13, 14, 16, 17, 18, 19, 21, 22, 23, 24, 26, 27, 28,
                29, 31, 32, 33, 34, 36, 37, 38, 39, 41, 42, 43, 44, 46, 47, 48, 49,
            ],
            vec![
                1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 41, 43, 45, 47,
                49,
            ],
            vec![
                1, 2, 4, 5, 7, 8, 10, 11, 13, 14, 16, 17, 19, 20, 22, 23, 25, 26, 28, 29, 31, 32,
                34, 35, 37, 38, 40, 41, 43, 44, 46, 47, 49, 50,
            ],
            vec![
                1, 3, 5, 9, 11, 13, 15, 17, 19, 23, 25, 27, 29, 31, 33, 37, 39, 41, 43, 45, 47,
            ],
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
                46, 47, 48, 49, 50,
            ],
            vec![1, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49],
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
                46, 47, 48, 49, 50,
            ],
            vec![
                1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43,
                45, 47, 49,
            ],
            vec![
                1, 2, 4, 5, 7, 8, 10, 13, 14, 16, 17, 19, 20, 23, 25, 26, 28, 29, 31, 32, 34, 35,
                37, 38, 40, 41, 43, 46, 47, 49, 50,
            ],
            vec![
                1, 3, 5, 7, 9, 11, 13, 15, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45,
                47, 49,
            ],
            vec![
                1, 2, 3, 4, 6, 8, 9, 11, 12, 13, 16, 17, 18, 19, 22, 23, 24, 26, 27, 29, 31, 32,
                33, 34, 36, 37, 38, 39, 41, 43, 44, 46, 47, 48,
            ],
            vec![
                1, 5, 7, 11, 13, 17, 19, 23, 25, 29, 31, 35, 37, 41, 43, 47, 49,
            ],
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 38, 39, 40, 41, 42, 43, 44, 45,
                46, 47, 48, 49, 50,
            ],
            vec![
                1, 3, 5, 7, 9, 11, 13, 15, 17, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45,
                47, 49,
            ],
            vec![
                1, 2, 4, 5, 7, 8, 10, 11, 14, 16, 17, 19, 20, 22, 23, 25, 28, 29, 31, 32, 34, 35,
                37, 38, 40, 41, 43, 44, 46, 47, 49, 50,
            ],
            vec![
                1, 3, 7, 9, 11, 13, 17, 19, 21, 23, 27, 29, 31, 33, 37, 39, 41, 43, 47, 49,
            ],
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 42, 43, 44, 45,
                46, 47, 48, 49, 50,
            ],
            vec![1, 5, 11, 13, 17, 19, 23, 25, 29, 31, 37, 41, 43, 47],
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 44, 45,
                46, 47, 48, 49, 50,
            ],
            vec![
                1, 3, 5, 7, 9, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 35, 37, 39, 41, 43, 45, 47,
                49,
            ],
            vec![
                1, 2, 4, 7, 8, 11, 13, 14, 16, 17, 19, 22, 23, 26, 28, 29, 31, 32, 34, 37, 38, 41,
                43, 44, 46, 47, 49,
            ],
            vec![
                1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45,
                47, 49,
            ],
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
                45, 46, 48, 49, 50,
            ],
            vec![
                1, 5, 7, 11, 13, 17, 19, 23, 25, 29, 31, 35, 37, 41, 43, 47, 49,
            ],
            vec![
                1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 15, 16, 17, 18, 19, 20, 22, 23, 24, 25, 26,
                27, 29, 30, 31, 32, 33, 34, 36, 37, 38, 39, 40, 41, 43, 44, 45, 46, 47, 48, 50,
            ],
            vec![
                1, 3, 7, 9, 11, 13, 17, 19, 21, 23, 27, 29, 31, 33, 37, 39, 41, 43, 47, 49,
            ],
        ];

        let mut g = vec![vec![]; nums.len()];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        fn dfs(
            g: &Vec<Vec<i32>>,
            nums: &Vec<i32>,
            curr_idx: i32,
            from_idx: i32,
            deepth: i32,
            deepth_idx: &mut Vec<(i32, i32)>,
            coprimes: &Vec<Vec<i32>>,
            res: &mut Vec<i32>,
        ) {
            let curr_val = nums[curr_idx as usize] as usize;
            let mut max_deepth = 0;
            for &cop in &coprimes[curr_val] {
                let (deepth, idx) = deepth_idx[cop as usize];
                if deepth > max_deepth {
                    max_deepth = deepth;
                    res[curr_idx as usize] = idx;
                }
            }
            let temp = deepth_idx[curr_val];
            deepth_idx[curr_val] = (deepth, curr_idx);
            for &next in &g[curr_idx as usize] {
                if next != from_idx {
                    dfs(
                        g,
                        nums,
                        next,
                        curr_idx,
                        deepth + 1,
                        deepth_idx,
                        coprimes,
                        res,
                    );
                }
            }
            deepth_idx[curr_val] = temp;
        }
        let mut res = vec![-1; nums.len()];
        let mut deepth_idx = vec![(0, 0); 51];
        dfs(&g, &nums, 0, 0, 0, &mut deepth_idx, &coprimes, &mut res);
        res
    }

    /**
     * 924. 尽量减少恶意软件的传播
     * 给出了一个由 n 个节点组成的网络，用 n × n 个邻接矩阵图 graph 表示。在节点网络中，当 graph[i][j] = 1 时，表示节点 i 能够直接连接到另一个节点 j。
     * 一些节点 initial 最初被恶意软件感染。
     * 只要两个节点直接连接，且其中至少一个节点受到恶意软件的感染，那么两个节点都将被恶意软件感染。这种恶意软件的传播将继续，直到没有更多的节点可以被这种方式感染。
     * 假设 M(initial) 是在恶意软件停止传播之后，整个网络中感染恶意软件的最终节点数。
     * 如果从 initial 中移除某一节点能够最小化 M(initial)， 返回该节点。如果有多个节点满足条件，就返回索引最小的节点。
     * 请注意，如果某个节点已从受感染节点的列表 initial 中删除，它以后仍有可能因恶意软件传播而受到感染。
     */
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        let n = graph.len();
        let mut init_node = vec![false; n];
        let mut result = initial[0];
        for i in initial {
            init_node[i as usize] = true;
            result = result.min(i);
        }
        let mut g = vec![vec![]; n];
        for (node, adj) in graph.iter().enumerate() {
            for (next, &access) in adj.iter().enumerate() {
                if access == 1 {
                    g[node].push(next);
                }
            }
        }
        fn dfs(
            curr: usize,
            from: usize,
            g: &Vec<Vec<usize>>,
            visited: &mut Vec<bool>,
            initial: &Vec<bool>,
            block_size: &mut i32,
            unique_init: &mut i32,
        ) {
            if !visited[curr] {
                visited[curr] = true;
                *block_size += 1;
                if initial[curr] {
                    if *unique_init == -1 {
                        *unique_init = curr as i32;
                    } else if *unique_init >= 0 {
                        *unique_init = -2;
                    }
                }
                for &next in &g[curr] {
                    if next != from {
                        dfs(next, curr, g, visited, initial, block_size, unique_init);
                    }
                }
            }
        }
        let mut max_block = 0;
        let mut visited = vec![false; n];
        for i in 0..n {
            if !visited[i] {
                let mut curr_block_size = 0;
                let mut unique_init = -1;
                dfs(
                    i,
                    i,
                    &g,
                    &mut visited,
                    &init_node,
                    &mut curr_block_size,
                    &mut unique_init,
                );
                if unique_init >= 0 {
                    if curr_block_size > max_block {
                        result = unique_init;
                        max_block = curr_block_size;
                    } else if curr_block_size == max_block {
                        result = result.min(unique_init);
                    }
                }
            }
        }
        result
    }
    /**
     * 928. 尽量减少恶意软件的传播 II
     * 给定一个由 n 个节点组成的网络，用 n x n 个邻接矩阵 graph 表示。在节点网络中，只有当 graph[i][j] = 1 时，节点 i 能够直接连接到另一个节点 j。
     * 一些节点 initial 最初被恶意软件感染。只要两个节点直接连接，且其中至少一个节点受到恶意软件的感染，那么两个节点都将被恶意软件感染。
     * 这种恶意软件的传播将继续，直到没有更多的节点可以被这种方式感染。
     * 假设 M(initial) 是在恶意软件停止传播之后，整个网络中感染恶意软件的最终节点数。
     * 我们可以从 initial 中删除一个节点，并完全移除该节点以及从该节点到任何其他节点的任何连接。
     * 请返回移除后能够使 M(initial) 最小化的节点。如果有多个节点满足条件，返回索引 最小的节点 。
     */
    pub fn min_malware_spread_ii(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let n = graph.len();
        let mut init_node = vec![false; n];
        let mut result = initial[0];
        for i in initial {
            init_node[i as usize] = true;
            result = result.min(i);
        }
        let mut g = vec![vec![]; n];
        for (node, adj) in graph.iter().enumerate() {
            for (next, &access) in adj.iter().enumerate() {
                if access == 1 {
                    g[node].push(next);
                }
            }
        }
        fn dfs(
            curr: usize,
            from: usize,
            g: &Vec<Vec<usize>>,
            visited: &mut Vec<bool>,
            initial: &Vec<bool>,
            block_size: &mut i32,
            unique_init: &mut i32,
        ) {
            if initial[curr] {
                if *unique_init == -1 {
                    *unique_init = curr as i32;
                } else if *unique_init >= 0 && *unique_init != curr as i32 {
                    *unique_init = -2;
                }
            } else if !visited[curr] {
                visited[curr] = true;
                *block_size += 1;
                for &next in &g[curr] {
                    if next != from {
                        dfs(next, curr, g, visited, initial, block_size, unique_init);
                    }
                }
            }
        }
        let mut init_connect_block = HashMap::new();
        let mut visited = vec![false; n];
        for i in 0..n {
            if !visited[i] {
                let mut curr_block_size = 0;
                let mut unique_init = -1;
                if init_node[i] {
                    curr_block_size = 1;
                    unique_init = i as i32;
                } else {
                    dfs(
                        i,
                        i,
                        &g,
                        &mut visited,
                        &init_node,
                        &mut curr_block_size,
                        &mut unique_init,
                    );
                }
                if unique_init >= 0 {
                    let count = init_connect_block.entry(unique_init).or_insert(0);
                    *count += curr_block_size;
                }
            }
        }
        let mut max_block = 0;
        init_connect_block.iter().for_each(|(&k, &v)| {
            if v > max_block {
                result = k;
                max_block = v;
            } else if v == max_block {
                result = result.min(k);
            }
        });
        result
    }
    /**
     * 2007. 从双倍数组中还原原数组
     * 一个整数数组 original 可以转变成一个 双倍 数组 changed ，转变方式为将 original 中每个元素 值乘以 2 加入数组中，然后将所有元素 随机打乱 。
     * 给你一个数组 changed ，如果 change 是 双倍 数组，那么请你返回 original数组，否则请返回空数组。original 的元素可以以 任意 顺序返回。
     */
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut count = HashMap::new();
        for num in changed {
            *count.entry(num).or_insert(0) += 1;
        }
        let mut res = vec![];
        if let Some(c) = count.remove(&0) {
            if c & 1 == 1 {
                return vec![];
            } else {
                res.extend(std::iter::repeat(0).take(c / 2));
            }
        }
        for mut num in count.keys().map(|&x| x).collect::<Vec<i32>>() {
            if num & 1 == 0 && count.contains_key(&(num / 2)) {
                continue;
            }
            while let Some(&c) = count.get(&num) {
                let double_c = count.get_mut(&(num * 2));
                if double_c.is_none() {
                    return vec![];
                }
                let double_c = double_c.unwrap();
                if *double_c < c {
                    return vec![];
                }
                *double_c -= c;
                res.extend(std::iter::repeat(num).take(c));
                if *double_c != 0 {
                    num *= 2;
                } else {
                    num *= 4;
                }
            }
        }
        res
    }
    /**
     * 1883. 准时抵达会议现场的最小跳过休息次数
     * 给你一个整数 hoursBefore ，表示你要前往会议所剩下的可用小时数。要想成功抵达会议现场，你必须途经 n 条道路。
     * 道路的长度用一个长度为 n 的整数数组 dist 表示，其中 dist[i] 表示第 i 条道路的长度（单位：千米）。
     * 另给你一个整数 speed ，表示你在道路上前进的速度（单位：千米每小时）。
     * 当你通过第 i 条路之后，就必须休息并等待，直到 下一个整数小时 才能开始继续通过下一条道路。
     * 注意：你不需要在通过最后一条道路后休息，因为那时你已经抵达会议现场。
     *     例如，如果你通过一条道路用去 1.4 小时，那你必须停下来等待，到 2 小时才可以继续通过下一条道路。
     *     如果通过一条道路恰好用去 2 小时，就无需等待，可以直接继续。
     * 然而，为了能准时到达，你可以选择 跳过 一些路的休息时间，这意味着你不必等待下一个整数小时。
     * 注意，这意味着与不跳过任何休息时间相比，你可能在不同时刻到达接下来的道路。
     *     例如，假设通过第 1 条道路用去 1.4 小时，且通过第 2 条道路用去 0.6 小时。
     *     跳过第 1 条道路的休息时间意味着你将会在恰好 2 小时完成通过第 2 条道路，且你能够立即开始通过第 3 条道路。
     * 返回准时抵达会议现场所需要的 最小跳过次数 ，如果 无法准时参会 ，返回 -1 。
     */
    pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        let n = dist.len();
        let mut min_dist = vec![0i64; n];
        for i in 0..n {
            let mut pre = 0;
            for j in 0..n - 1 {
                let temp = min_dist[j + 1];
                min_dist[j + 1] = ((min_dist[j] + dist[j] as i64) as f64 / speed as f64).ceil()
                    as i64
                    * speed as i64;
                if i != 0 {
                    min_dist[j + 1] = min_dist[j + 1].min(pre + dist[j] as i64);
                }
                pre = temp;
            }
            if min_dist[n - 1] + dist[n - 1] as i64 <= hours_before as i64 * speed as i64 {
                return i as i32;
            }
        }
        -1
    }
    /**
     * 39. 组合总和
     * 给你一个 无重复元素 的整数数组 candidates 和一个目标整数 target ，
     * 找出 candidates 中可以使数字和为目标数 target 的 所有 不同组合 ，并以列表形式返回。你可以按 任意顺序 返回这些组合。
     * candidates 中的 同一个 数字可以 无限制重复被选取 。如果至少一个数字的被选数量不同，则两种组合是不同的。
     * 对于给定的输入，保证和为 target 的不同组合数少于 150 个。
     */
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut res = vec![];
        fn dfs(
            i: usize,
            left: i32,
            candidates: &Vec<i32>,
            comb: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if left == 0 {
                res.push(comb.clone());
                return;
            }
            if i == candidates.len() || left < 0 || left < candidates[i] {
                return;
            }
            dfs(i + 1, left, candidates, comb, res);
            comb.push(candidates[i]);
            dfs(i, left - candidates[i], candidates, comb, res);
            comb.pop();
        }
        dfs(0, target, &candidates, &mut vec![], &mut res);
        res
    }
    /**
     * 2462. 雇佣 K 位工人的总代价
     * 给你一个下标从 0 开始的整数数组 costs ，其中 costs[i] 是雇佣第 i 位工人的代价。
     * 同时给你两个整数 k 和 candidates 。我们想根据以下规则恰好雇佣 k 位工人：
     *     总共进行 k 轮雇佣，且每一轮恰好雇佣一位工人。
     *     在每一轮雇佣中，从最前面 candidates 和最后面 candidates 人中选出代价最小的一位工人，如果有多位代价相同且最小的工人，选择下标更小的一位工人。
     *         比方说，costs = [3,2,7,7,1,2] 且 candidates = 2 ，第一轮雇佣中，我们选择第 4 位工人，因为他的代价最小 [3,2,7,7,1,2] 。
     *         第二轮雇佣，我们选择第 1 位工人，因为他们的代价与第 4 位工人一样都是最小代价，而且下标更小，[3,2,7,7,2] 。注意每一轮雇佣后，剩余工人的下标可能会发生变化。
     *     如果剩余员工数目不足 candidates 人，那么下一轮雇佣他们中代价最小的一人，如果有多位代价相同且最小的工人，选择下标更小的一位工人。
     *     一位工人只能被选择一次。
     * 返回雇佣恰好 k 位工人的总代价。
     */
    pub fn total_cost(mut costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let n = costs.len();
        if k as usize >= n {
            return costs.iter().map(|&c| c as i64).sum();
        }
        if candidates as usize * 2 >= n {
            costs.sort_unstable();
            return costs.iter().take(k as usize).map(|&c| c as i64).sum();
        }
        let mut prefix =
            BinaryHeap::from_iter(costs.iter().take(candidates as usize).map(|&c| Reverse(c)));
        let mut suffix = BinaryHeap::from_iter(
            costs
                .iter()
                .rev()
                .take(candidates as usize)
                .map(|&c| Reverse(c)),
        );
        let mut pre_end = candidates - 1;
        let mut suf_start = n as i32 - candidates - 1;
        let mut res = 0;
        for _ in 0..k {
            if prefix.is_empty() {
                res += suffix.pop().unwrap().0 as i64;
            } else if suffix.is_empty() {
                res += prefix.pop().unwrap().0 as i64;
            } else if prefix.peek().unwrap().0 <= suffix.peek().unwrap().0 {
                res += prefix.pop().unwrap().0 as i64;
                if pre_end < suf_start {
                    pre_end += 1;
                    prefix.push(Reverse(costs[pre_end as usize]));
                }
            } else {
                res += suffix.pop().unwrap().0 as i64;
                if pre_end < suf_start {
                    suffix.push(Reverse(costs[suf_start as usize]));
                    suf_start -= 1;
                }
            }
        }
        res
    }
    /**
     * 857. 雇佣 K 名工人的最低成本
     * 有 n 名工人。 给定两个数组 quality 和 wage ，其中，quality[i] 表示第 i 名工人的工作质量，其最低期望工资为 wage[i] 。
     * 现在我们想雇佣 k 名工人组成一个工资组。在雇佣 一组 k 名工人时，我们必须按照下述规则向他们支付工资：
     *     对工资组中的每名工人，应当按其工作质量与同组其他工人的工作质量的比例来支付工资。
     *     工资组中的每名工人至少应当得到他们的最低期望工资。
     * 给定整数 k ，返回 组成满足上述条件的付费群体所需的最小金额 。在实际答案的 10^-5 以内的答案将被接受。
     */
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        use std::collections::BinaryHeap;
        let mut id = (0..quality.len()).collect::<Vec<_>>();
        id.sort_unstable_by(|&a, &b| {
            (wage[a] as f64 / quality[a] as f64).total_cmp(&(wage[b] as f64 / quality[b] as f64))
        });
        let mut qual_heap = BinaryHeap::new();
        let mut qual_sum = 0;
        for &idx in id.iter().take(k as usize) {
            qual_heap.push(quality[idx]);
            qual_sum += quality[idx];
        }
        let mut res = (qual_sum as f64)
            * (wage[id[k as usize - 1]] as f64 / quality[id[k as usize - 1]] as f64);
        for &idx in &id[k as usize..] {
            let q = quality[idx];
            if q < *qual_heap.peek().unwrap() {
                qual_sum = qual_sum - qual_heap.pop().unwrap() + q;
                qual_heap.push(q);
                res = res.min((qual_sum as f64) * (wage[idx] as f64 / quality[idx] as f64));
            }
        }
        res
    }
    /**
     * 1235. 规划兼职工作
     * 你打算利用空闲时间来做兼职工作赚些零花钱。
     * 这里有 n 份兼职工作，每份工作预计从 startTime[i] 开始到 endTime[i] 结束，报酬为 profit[i]。
     * 给你一份兼职工作表，包含开始时间 startTime，结束时间 endTime 和预计报酬 profit 三个数组，请你计算并返回可以获得的最大报酬。
     * 注意，时间上出现重叠的 2 份工作不能同时进行。
     * 如果你选择的工作在时间 X 结束，那么你可以立刻进行在时间 X 开始的下一份工作。
     */
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut info = start_time
            .iter()
            .zip(end_time.iter())
            .zip(profit.iter())
            .map(|((&a, &b), &c)| (a, b, c))
            .collect::<Vec<_>>();
        info.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        let mut memory = vec![0; n + 1];
        for i in 0..n {
            let curr_start = info[i].0;
            let pre = info.partition_point(|x| x.1 <= curr_start);
            memory[i + 1] = memory[i].max(info[i].2 + memory[pre]);
        }
        memory[n]
    }
    /**
     * 1463. 摘樱桃 II
     * 给你一个 rows x cols 的矩阵 grid 来表示一块樱桃地。 grid 中每个格子的数字表示你能获得的樱桃数目。
     * 你有两个机器人帮你收集樱桃，机器人 1 从左上角格子 (0,0) 出发，机器人 2 从右上角格子 (0, cols-1) 出发。
     * 请你按照如下规则，返回两个机器人能收集的最多樱桃数目：
     *     从格子 (i,j) 出发，机器人可以移动到格子 (i+1, j-1)，(i+1, j) 或者 (i+1, j+1) 。
     *     当一个机器人经过某个格子时，它会把该格子内所有的樱桃都摘走，然后这个位置会变成空格子，即没有樱桃的格子。
     *     当两个机器人同时到达同一个格子时，它们中只有一个可以摘到樱桃。
     *     两个机器人在任意时刻都不能移动到 grid 外面。
     *     两个机器人最后都要到达 grid 最底下一行。
     */
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let row = grid.len();
        let col = grid[0].len();
        let mut front = vec![vec![0; col + 2]; col + 2];
        let mut current = vec![vec![0; col + 2]; col + 2];
        // step 0, robot #1 in column 0, robot #2 in column col-1
        let mut res = 0;
        front[1][col] = grid[0][0] + grid[0][col - 1];
        for step in 1..row {
            for i in 1..(col + 1).min(step + 2) {
                for j in col.saturating_sub(step).max(i + 1)..=col {
                    current[i][j] = *[
                        front[i - 1][j - 1],
                        front[i][j - 1],
                        front[i + 1][j - 1],
                        front[i - 1][j],
                        front[i][j],
                        front[i + 1][j],
                        front[i - 1][j + 1],
                        front[i][j + 1],
                        front[i + 1][j + 1],
                    ]
                    .iter()
                    .max()
                    .unwrap()
                        + grid[step][i - 1]
                        + grid[step][j - 1];
                    res = res.max(current[i][j]);
                }
            }
            std::mem::swap(&mut current, &mut front);
        }
        res
    }
    /**
     * 1553. 吃掉 N 个橘子的最少天数
     * 厨房里总共有 n 个橘子，你决定每一天选择如下方式之一吃这些橘子：
     *     吃掉一个橘子。
     *     如果剩余橘子数 n 能被 2 整除，那么你可以吃掉 n/2 个橘子。
     *     如果剩余橘子数 n 能被 3 整除，那么你可以吃掉 2*(n/3) 个橘子。
     * 每天你只能从以上 3 种方案中选择一种方案。
     * 请你返回吃掉所有 n 个橘子的最少天数。
     */
    pub fn min_days(n: i32) -> i32 {
        use std::collections::HashMap;
        let mut memory = HashMap::new();
        fn dp(curr: i32, memory: &mut HashMap<i32, i32>) -> i32 {
            if curr <= 1 {
                return curr;
            }
            if let Some(&result) = memory.get(&curr) {
                return result;
            }
            let result = (curr % 2 + dp(curr / 2, memory)).min(curr % 3 + dp(curr / 3, memory)) + 1;
            memory.insert(curr, result);
            result
        }
        dp(n, &mut memory)
    }
    /**
     * 2589. 完成所有任务的最少时间
     * 你有一台电脑，它可以 同时 运行无数个任务。
     * 给你一个二维整数数组 tasks ，其中 tasks[i] = [starti, endi, durationi] 表示
     * 第 i 个任务需要在 闭区间 时间段 [starti, endi] 内运行 durationi 个整数时间点（但不需要连续）。
     * 当电脑需要运行任务时，你可以打开电脑，如果空闲时，你可以将电脑关闭。
     * 请你返回完成所有任务的情况下，电脑最少需要运行多少秒。
     */
    pub fn find_minimum_time(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut stack = vec![(-1, -1, 0)];
        for task in tasks.iter() {
            let start = task[0];
            let end = task[1];
            let mut duration = task[2];
            let mut first_overlap = stack.partition_point(|x| x.0 < start) - 1;
            duration -= stack.last().unwrap().2 - stack[first_overlap].2;
            if start <= stack[first_overlap].1 {
                duration -= stack[first_overlap].1 - start + 1;
            }
            if duration <= 0 {
                continue;
            }
            while end - stack.last().unwrap().1 <= duration {
                let (st, ed, _) = stack.pop().unwrap();
                duration += ed - st + 1;
            }
            stack.push((end - duration + 1, end, stack.last().unwrap().2 + duration));
        }
        stack.last().unwrap().2
    }
    /**
     * 1542. 找出最长的超赞子字符串
     * 给你一个字符串 s 。请返回 s 中最长的 超赞子字符串 的长度。
     * 「超赞子字符串」需满足满足下述两个条件：
     *     该字符串是 s 的一个非空子字符串
     *     进行任意次数的字符交换后，该字符串可以变成一个回文字符串
     */
    pub fn longest_awesome(s: String) -> i32 {
        use std::collections::HashMap;
        let mut location = HashMap::new();
        let mut res = 0;
        let mut status = 0;
        location.insert(0, -1);
        let s = s.as_bytes();
        for i in 0..s.len() {
            status ^= (1 << (s[i] - b'0') as i32);
            for j in 0..10 {
                let pre = status ^ (1 << j);
                if let Some(&idx) = location.get(&pre) {
                    res = res.max(i as i32 - idx);
                }
            }
            if let Some(&idx) = location.get(&status) {
                res = res.max(i as i32 - idx);
            } else {
                location.insert(status, i as i32);
            }
        }
        res
    }
    /**
     * 1328. 破坏回文串
     * 给你一个由小写英文字母组成的回文字符串 palindrome ，
     * 请你将其中一个字符用任意小写英文字母替换，使得结果字符串的字典序最小 ，且不是回文串。
     * 请你返回结果字符串。如果无法做到，则返回一个空串 。
     * 如果两个字符串长度相同，那么字符串 a 字典序比字符串 b 小可以这样定义：
     * 在 a 和 b 出现不同的第一个位置上，字符串 a 中的字符严格小于 b 中的对应字符。
     * 例如，"abcc” 字典序比 "abcd" 小，因为不同的第一个位置是在第四个字符，显然 'c' 比 'd' 小。
     */
    pub fn break_palindrome(mut palindrome: String) -> String {
        let n = palindrome.len();
        if n <= 1 {
            return String::from("");
        }
        let char_arr = unsafe { palindrome.as_bytes_mut() };
        for (idx, c) in char_arr.iter_mut().enumerate() {
            if *c != 'a' as u8 && !(n % 2 == 1 && idx == n / 2) {
                *c = 'a' as u8;
                return String::from_utf8(char_arr.to_vec()).unwrap();
            }
        }
        char_arr[char_arr.len() - 1] = 'b' as u8;
        String::from_utf8(char_arr.to_vec()).unwrap()
    }
    /**
     * 31. 下一个排列
     * 给你一个整数数组 nums ，找出 nums 的下一个排列。
     * 必须 原地 修改，只允许使用额外常数空间。
     */
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut sorted_locate = 0usize;
        for i in (1..n).rev() {
            if nums[i] > nums[i - 1] {
                sorted_locate = i;
                break;
            }
        }
        if sorted_locate == 0 {
            nums.reverse();
        } else {
            let mut change_locate = sorted_locate;
            for i in (sorted_locate..n).rev() {
                if nums[i] > nums[sorted_locate - 1] {
                    change_locate = i;
                    break;
                }
            }
            nums.swap(sorted_locate - 1, change_locate);
            nums[sorted_locate..].reverse();
        }
    }
    /**
     * 157. 套餐内商品的排列顺序
     * 某店铺将用于组成套餐的商品记作字符串 goods，其中 goods[i] 表示对应商品。请返回该套餐内所含商品的 全部排列方式 。
     * 返回结果 无顺序要求，但不能含有重复的元素。
     */
    pub fn goods_order(mut goods: String) -> Vec<String> {
        let n = goods.len();
        let goods = unsafe { goods.as_bytes_mut() };
        goods.sort_unstable();
        let mut result = vec![String::from_utf8(goods.to_vec()).unwrap()];
        let mut sorted_locate = 1;
        while sorted_locate != 0 {
            sorted_locate = 0;
            for i in (1..n).rev() {
                if goods[i] > goods[i - 1] {
                    sorted_locate = i;
                    break;
                }
            }
            if sorted_locate != 0 {
                let mut change_locate = sorted_locate;
                for i in (sorted_locate..n).rev() {
                    if goods[i] > goods[sorted_locate - 1] {
                        change_locate = i;
                        break;
                    }
                }
                goods.swap(sorted_locate - 1, change_locate);
                goods[sorted_locate..].reverse();
                result.push(String::from_utf8(goods.to_vec()).unwrap());
            }
        }
        result
    }
    /**
     * 2588. 统计美丽子数组数目
     * 给你一个下标从 0 开始的整数数组nums 。每次操作中，你可以：
     *     选择两个满足 0 <= i, j < nums.length 的不同下标 i 和 j 。
     *     选择一个非负整数 k ，满足 nums[i] 和 nums[j] 在二进制下的第 k 位（下标编号从 0 开始）是 1 。
     *     将 nums[i] 和 nums[j] 都减去 2^k 。
     * 如果一个子数组内执行上述操作若干次后，该子数组可以变成一个全为 0 的数组，那么我们称它是一个 美丽 的子数组。
     * 请你返回数组 nums 中 美丽子数组 的数目。
     * 子数组是一个数组中一段连续 非空 的元素序列。
     */
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        use std::collections::HashMap;
        let mut counter = HashMap::new();
        let mut flag = 0;
        let mut result = 0;
        counter.insert(0, 1);
        for n in nums {
            flag ^= n;
            if let Some(count) = counter.get_mut(&flag) {
                result += *count;
                *count += 1;
            } else {
                counter.insert(flag, 1);
            }
        }
        result
    }
}
