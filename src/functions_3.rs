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
}
