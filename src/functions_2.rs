use std::collections::HashMap;

pub struct Solution;

#[allow(unused)]
impl Solution {
    /**
     * 37. 解数独
     * 编写一个程序，通过填充空格来解决数独问题。
     * 数独的解法需 遵循如下规则：
     *     数字 1-9 在每一行只能出现一次。
     *     数字 1-9 在每一列只能出现一次。
     *     数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。
     * 数独部分空格内已填入了数字，空白格用 '.' 表示。
     */
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut converted_soduku = Vec::with_capacity(81);
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    converted_soduku.push(0);
                } else {
                    converted_soduku.push(board[i][j] as u8 - '0' as u8);
                }
            }
        }
        fn check(soduku: &Vec<u8>, put_num: (usize, usize, u8)) -> bool {
            let row = put_num.0;
            let col = put_num.1;
            let val = put_num.2;
            // check row
            for i in row * 9..row * 9 + 9 {
                if soduku[i] == val {
                    return false;
                }
            }
            // check column
            for i in (col..80).step_by(9) {
                if soduku[i] == val {
                    return false;
                }
            }
            // check area
            for i in row / 3 * 3..row / 3 * 3 + 3 {
                for j in col / 3 * 3..col / 3 * 3 + 3 {
                    if soduku[i * 9 + j] == val {
                        return false;
                    }
                }
            }
            true
        }
        fn solve(curr_soduku: &mut Vec<u8>) -> bool {
            // find empty cell
            let mut empty_cell = 0;
            while empty_cell < 81 && curr_soduku[empty_cell] != 0 {
                empty_cell += 1;
            }
            if empty_cell == 81 {
                return true;
            }
            for val in 1..=9 {
                if check(curr_soduku, (empty_cell / 9, empty_cell % 9, val)) {
                    curr_soduku[empty_cell] = val;
                    if solve(curr_soduku) {
                        return true;
                    }
                    curr_soduku[empty_cell] = 0;
                }
            }
            return false;
        }

        solve(&mut converted_soduku);

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    board[i][j] = char::from(converted_soduku[i * 9 + j] + '0' as u8);
                }
            }
        }
    }

    /**
     * 28. 找出字符串中第一个匹配项的下标
     *给你两个字符串 haystack 和 needle，
     *请你在 haystack 字符串中找出 needle 字符串的第一个匹配项的下标（下标从 0 开始）。
     *如果 needle 不是 haystack 的一部分，则返回  -1 。
     */
    pub fn str_str(haystack: String, needle: String) -> i32 {
        //1、部分匹配值表
        let mut vector = vec![0; needle.len()];
        //由于只对英文进行处理，所以直接用字节数组，否则无法随机访问
        let needle_bytes = needle.as_bytes();
        let mut i = 0;
        let mut j = 1;
        let mut same_char = 1;
        while j < needle.len() {
            if needle_bytes[i] == needle_bytes[j] {
                vector[j] = same_char;
                i += 1;
                j += 1;
                same_char += 1;
            } else {
                same_char = 1;
                if i == 0 {
                    j += 1;
                } else {
                    //从不一样的位置往前比较，看是否有相同的子串
                    let mut a = 0;
                    i -= 1;
                    while needle_bytes[i] == needle_bytes[j - a] {
                        if i == 0 {
                            vector[j] = a + 1;
                            j += 1;
                            break;
                        }
                        i -= 1;
                        a += 1;
                    }
                    i = 0;
                }
            }
        }
        let haystack_bytes = haystack.as_bytes();
        i = 0; //起始比较位置
        j = 0; //当前比较位置
        while i + j < haystack.len() {
            if haystack_bytes[i + j] == needle_bytes[j] {
                j += 1;
                if j == needle_bytes.len() {
                    return i as i32;
                }
            } else {
                if j == 0 || vector[j - 1] == 0 {
                    i += 1;
                    j = 0;
                } else {
                    i = i + j - vector[j - 1];
                    j = vector[j - 1];
                }
            }
        }
        -1
    }

    /**
     * 2300. 咒语和药水的成功对数
     * 给你两个正整数数组 spells 和 potions ，长度分别为 n 和 m ，其中 spells[i] 表示第 i 个咒语的能量强度，potions[j] 表示第 j 瓶药水的能量强度。
     * 同时给你一个整数 success 。一个咒语和药水的能量强度 相乘 如果 大于等于 success ，那么它们视为一对 成功 的组合。
     * 请你返回一个长度为 n 的整数数组 pairs，其中 pairs[i] 是能跟第 i 个咒语成功组合的 药水 数目。
     */
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions;
        let psize = potions.len() as isize;
        potions.sort();
        let mut pairs: Vec<i32> = Vec::new();
        for s in spells {
            let mut left: isize = 0;
            let mut right: isize = psize - 1;
            let mut mid: isize = (left + right) / 2;
            while left <= right {
                let energy: i64 = s as i64 * potions[mid as usize] as i64;
                if energy < success {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
                mid = (left + right) / 2;
            }
            pairs.push((psize - right - 1) as i32);
        }
        pairs
    }

    /**
     * 16. 最接近的三数之和
     * 给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。
     * 返回这三个数的和。
     * 假定每组输入只存在恰好一个解。
     */
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut changable = nums.clone();
        changable.sort();
        let size = nums.len();
        let mut left;
        let mut right;
        let mut distance: i32 = 0x7FFFFFFF;
        for i in 0..size - 2 {
            left = i + 1;
            right = size - 1;
            while left < right {
                let temp = target - changable[i] - changable[left] - changable[right];
                if temp == 0 {
                    return target;
                } else if temp < 0 {
                    right -= 1;
                } else {
                    left += 1;
                }
                if temp.abs() < distance.abs() {
                    distance = temp;
                }
            }
        }
        target - distance
    }

    /**
     * 给你一个由 不同 正整数组成的数组 nums ，
     * 请你返回满足 a * b = c * d 的元组 (a, b, c, d) 的数量。
     * 其中 a、b、c 和 d 都是 nums 中的元素，且 a != b != c != d 。
     */
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        fn get_c(m: i32, n: i32) -> i32 {
            let mut m_factorial = 1;
            let mut n_factorial = 1;
            let mut sub_mn_factorial = 1;
            for i in 1..m + 1 {
                m_factorial *= i;
            }
            for i in 1..n + 1 {
                n_factorial *= i;
            }
            for i in 1..m - n + 1 {
                sub_mn_factorial *= i;
            }
            m_factorial / (n_factorial * sub_mn_factorial)
        }
        let mut truple_map: HashMap<i32, i32> = HashMap::new();
        let size = nums.len();
        let mut result = 0;
        for i in 0..(size - 1) {
            for j in (i + 1)..size {
                let count = truple_map.entry(nums[i] * nums[j]).or_insert(0);
                *count += 1;
            }
        }
        for v in truple_map.values() {
            if *v > 1 {
                result += get_c(*v, 2) * 8;
            }
        }
        result
    }

    /**
     * 204. 计数质数
     * 给定整数 n ，返回 所有小于非负整数 n 的质数的数量 。
     */
    pub fn count_primes(n: i32) -> i32 {
        let mut primes = Vec::new();
        let mut flags = vec![true; n as usize + 1];
        for i in 2..=n as usize {
            if flags[i] {
                primes.push(i);
            }
            for &p in primes.iter() {
                if p * i > n as usize {
                    break;
                }
                flags[p * i] = false;
                if i % p == 0 {
                    break;
                }
            }
        }
        primes.len() as i32
    }

    /**
     * 给你一棵 n 个节点的无向树，节点编号为 1 到 n 。
     * 给你一个整数 n 和一个长度为 n - 1 的二维整数数组 edges ，其中 edges[i] = [ui, vi] 表示节点 ui 和 vi 在树中有一条边。
     * 请你返回树中的 合法路径数目 。
     * 如果在节点 a 到节点 b 之间 恰好有一个 节点的编号是质数，那么我们称路径 (a, b) 是 合法的 。
     * 注意：
     *     路径 (a, b) 指的是一条从节点 a 开始到节点 b 结束的一个节点序列，序列中的节点 互不相同 ，且相邻节点之间在树上有一条边。
     *     路径 (a, b) 和路径 (b, a) 视为 同一条 路径，且只计入答案 一次 。
     */
    pub fn count_paths(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        // 找素数，欧拉筛
        let mut primes = Vec::new();
        let mut is_primes = vec![true; n as usize + 1];
        is_primes[1] = false;
        for i in 2..=n {
            if is_primes[i as usize] {
                primes.push(i);
            }
            for &p in primes.iter() {
                if p * i > n {
                    break;
                }
                is_primes[(p * i) as usize] = false;
                if i % p == 0 {
                    break;
                }
            }
        }
        // 建图
        let mut g = vec![vec![]; n as usize + 1];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        // 统计不含质数的子图节点
        fn dfs(
            g: &Vec<Vec<i32>>,
            from: i32,
            curr: i32,
            is_primes: &Vec<bool>,
            curr_g: &mut Vec<i32>,
        ) {
            curr_g.push(curr);
            for &e in g[curr as usize].iter() {
                if !is_primes[e as usize] && e != from {
                    dfs(g, curr, e, is_primes, curr_g);
                }
            }
        }
        let mut child_g = Vec::new();
        let mut child_g_sizes = vec![0; n as usize + 1];
        let mut ret = 0;
        for p in primes {
            // 求所有邻接且不包含质数的子图的大小
            let adj = &g[p as usize];
            for &e in adj {
                if child_g_sizes[e as usize] == 0 && !is_primes[e as usize] {
                    dfs(&g, p, e, &is_primes, &mut child_g);
                    for &n in child_g.iter() {
                        child_g_sizes[n as usize] = child_g.len() as i32;
                    }
                    child_g.clear();
                }
            }
            let mut front_sum = 0;
            for i in 0..adj.len() {
                ret += front_sum * child_g_sizes[adj[i] as usize] as i64;
                front_sum += child_g_sizes[adj[i] as usize] as i64;
            }
            ret += front_sum;
        }
        ret
    }

    /**
     * 2673. 使二叉树所有路径值相等的最小代价
     * 给你一个整数 n 表示一棵 满二叉树 里面节点的数目，节点编号从 1 到 n 。
     * 根节点编号为 1 ，树中每个非叶子节点 i 都有两个孩子，分别是左孩子 2 * i 和右孩子 2 * i + 1 。
     * 树中每个节点都有一个值，用下标从 0 开始、长度为 n 的整数数组 cost 表示，
     * 其中 cost[i] 是第 i + 1 个节点的值。每次操作，你可以将树中 任意 节点的值 增加 1 。你可以执行操作 任意 次。
     * 你的目标是让根到每一个 叶子结点 的路径值相等。请你返回 最少 需要执行增加操作多少次。
     * 注意：
     *     满二叉树 指的是一棵树，它满足树中除了叶子节点外每个节点都恰好有 2 个子节点，且所有叶子节点距离根节点距离相同。
     *     路径值 指的是路径上所有节点的值之和。
     */
    pub fn min_increments(n: i32, mut cost: Vec<i32>) -> i32 {
        let mut ret = 0;
        for i in (0..cost.len() / 2).rev() {
            ret += (cost[i * 2 + 1] - cost[i * 2 + 2]).abs();
            cost[i] += cost[i * 2 + 1].max(cost[i * 2 + 2]);
        }
        ret
    }

    /**
     * 834. 树中距离之和
     * 给定一个无向、连通的树。树中有 n 个标记为 0...n-1 的节点以及 n-1 条边 。
     * 给定整数 n 和数组 edges ， edges[i] = [ai, bi]表示树中的节点 ai 和 bi 之间有一条边。
     * 返回长度为 n 的数组 answer ，其中 answer[i] 是树中第 i 个节点与所有其他节点之间的距离之和。
     */
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        // 建图
        let mut g = vec![vec![]; n as usize];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        // 以0为根，求各节点到0的距离之和，以及各节点的子树大小
        fn dfs(
            g: &Vec<Vec<i32>>,
            curr: i32,
            from: i32,
            curr_distance: i32,
            sub_tree_size: &mut Vec<i32>,
        ) -> i32 {
            let mut ret = curr_distance;
            for &n in &g[curr as usize] {
                if n != from {
                    ret += dfs(g, n, curr, curr_distance + 1, sub_tree_size);
                    sub_tree_size[curr as usize] += sub_tree_size[n as usize];
                }
            }
            sub_tree_size[curr as usize] += 1;
            ret
        }
        let mut distance = vec![0; n as usize];
        let mut sub_tree_size = vec![0; n as usize];
        distance[0] = dfs(&g, 0, 0, 0, &mut sub_tree_size);

        // 求结果
        fn slove(
            g: &Vec<Vec<i32>>,
            curr: i32,
            from: i32,
            tree_size: i32,
            distance: &mut Vec<i32>,
            sub_tree_size: &Vec<i32>,
        ) {
            for &n in &g[curr as usize] {
                if n != from {
                    distance[n as usize] =
                        distance[curr as usize] + tree_size - sub_tree_size[n as usize] * 2;
                    slove(g, n, curr, tree_size, distance, sub_tree_size);
                }
            }
        }
        slove(&g, 0, 0, n, &mut distance, &sub_tree_size);
        distance
    }

    /**
     * 2581. 统计可能的树根数目
     * Alice 有一棵 n 个节点的树，节点编号为 0 到 n - 1 。
     * 树用一个长度为 n - 1 的二维整数数组 edges 表示，其中 edges[i] = [ai, bi] ，表示树中节点 ai 和 bi 之间有一条边。
     * Alice 想要 Bob 找到这棵树的根。她允许 Bob 对这棵树进行若干次 猜测 。每一次猜测，Bob 做如下事情：
     *     选择两个 不相等 的整数 u 和 v ，且树中必须存在边 [u, v] 。
     *     Bob 猜测树中 u 是 v 的 父节点 。
     * Bob 的猜测用二维整数数组 guesses 表示，其中 guesses[j] = [uj, vj] 表示 Bob 猜 uj 是 vj 的父节点。
     * Alice 非常懒，她不想逐个回答 Bob 的猜测，只告诉 Bob 这些猜测里面 至少 有 k 个猜测的结果为 true 。
     * 给你二维整数数组 edges ，Bob 的所有猜测和整数 k ，请你返回可能成为树根的 节点数目 。如果没有这样的树，则返回 0。
     */
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
        // 建图
        use std::collections::HashMap;
        let mut g = vec![vec![]; edges.len() + 1];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        let mut map = HashMap::with_capacity(guesses.len());
        for guess in guesses {
            let v = map.entry(guess[0]).or_insert(Vec::new());
            v.push(guess[1]);
        }
        // 获取以0为根节点时，猜测正确的数量
        let is_guess_match = |u: i32, v: i32| {
            if map.get(&u).is_some_and(|vec| vec.contains(&v)) {
                true
            } else {
                false
            }
        };
        fn get_matched(
            g: &Vec<Vec<i32>>,
            curr: i32,
            from: i32,
            matched: &mut i32,
            is_guess_match: &impl Fn(i32, i32) -> bool,
        ) {
            if is_guess_match(from, curr) {
                *matched += 1;
            }
            for &n in &g[curr as usize] {
                if n != from {
                    get_matched(g, n, curr, matched, is_guess_match);
                }
            }
        }
        let mut matched = 0;
        get_matched(&g, 0, 0, &mut matched, &is_guess_match);
        // 求猜测正确数量为k的根节点个数
        fn slove(
            g: &Vec<Vec<i32>>,
            curr: i32,
            from: i32,
            matched: i32,
            k: i32,
            result: &mut i32,
            is_guess_match: &impl Fn(i32, i32) -> bool,
        ) {
            let mut curr_matched = matched;
            if is_guess_match(curr, from) {
                curr_matched += 1;
            }
            if is_guess_match(from, curr) {
                curr_matched -= 1;
            }
            if curr_matched >= k {
                *result += 1;
            }
            for &n in &g[curr as usize] {
                if n != from {
                    slove(g, n, curr, curr_matched, k, result, is_guess_match);
                }
            }
        }
        let mut ret = 0;
        slove(&g, 0, 0, matched, k, &mut ret, &is_guess_match);
        ret
    }

    /**
     * 2369. 检查数组是否存在有效划分
     * 给你一个下标从 0 开始的整数数组 nums ，你必须将数组划分为一个或多个 连续 子数组。
     * 如果获得的这些子数组中每个都能满足下述条件 之一 ，则可以称其为数组的一种 有效 划分：
     *     子数组 恰 由 2 个相等元素组成，例如，子数组 [2,2] 。
     *     子数组 恰 由 3 个相等元素组成，例如，子数组 [4,4,4] 。
     *     子数组 恰 由 3 个连续递增元素组成，并且相邻元素之间的差值为 1 。例如，子数组 [3,4,5] ，但是子数组 [1,3,5] 不符合要求。
     * 如果数组 至少 存在一种有效划分，返回 true ，否则，返回 false 。
     */
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let mut memory = vec![false; nums.len() + 1];
        memory[0] = true;
        for i in 1..nums.len() {
            if (memory[i - 1] && nums[i] == nums[i - 1])
                || (i > 1
                    && memory[i - 2]
                    && ((nums[i] == nums[i - 1] && nums[i] == nums[i - 2])
                        || (nums[i] - nums[i - 1] == 1 && nums[i - 1] - nums[i - 2] == 1)))
            {
                memory[i + 1] = true;
            }
        }
        memory[nums.len()]
    }

    /**
     * 1976. 到达目的地的方案数
     * 你在一个城市里，城市由 n 个路口组成，路口编号为 0 到 n - 1 ，
     * 某些路口之间有 双向 道路。输入保证你可以从任意路口出发到达其他任意路口，且任意两个路口之间最多有一条路。
     * 给你一个整数 n 和二维整数数组 roads ，其中 roads[i] = [ui, vi, timei] 表示在路口 ui 和 vi 之间有一条需要花费 timei 时间才能通过的道路。
     * 你想知道花费 最少时间 从路口 0 出发到达路口 n - 1 的方案数。
     * 请返回花费 最少时间 到达目的地的 路径数目 。由于答案可能很大，将结果对 109 + 7 取余 后返回。
     */
    pub fn count_paths_1976(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut adj = vec![vec![]; n as usize];
        for r in roads {
            adj[r[0] as usize].push((r[1], r[2]));
            adj[r[1] as usize].push((r[0], r[2]));
        }
        let mut distance = vec![i64::MAX; n as usize];
        let mut visited = vec![false; n as usize];
        let mut min = BinaryHeap::new();
        let mut ways = vec![0; n as usize];
        ways[0] = 1;
        distance[0] = 0;
        min.push((Reverse(0), 0));
        while let Some((_, curr)) = min.pop() {
            let curr = curr as usize;
            if visited[curr] {
                continue;
            }
            visited[curr] = true;
            for &adj_nodes in &adj[curr] {
                let node = adj_nodes.0 as usize;
                let dis = adj_nodes.1 as i64;
                if distance[curr] + dis < distance[node] {
                    distance[node] = distance[curr] + dis;
                    min.push((Reverse(distance[node]), adj_nodes.0));
                    ways[node] = ways[curr];
                } else if distance[curr] + dis == distance[node] {
                    ways[node] = (ways[curr] + ways[node]) % 1_000_000_007;
                }
            }
        }
        ways[n as usize - 1]
    }

    /**
     * 2575. 找出字符串的可整除数组
     * 给你一个下标从 0 开始的字符串 word ，长度为 n ，由从 0 到 9 的数字组成。另给你一个正整数 m 。
     * word 的 可整除数组 div  是一个长度为 n 的整数数组，并满足：
     *     如果 word[0,...,i] 所表示的 数值 能被 m 整除，div[i] = 1
     *     否则，div[i] = 0
     * 返回 word 的可整除数组。
     */
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let mut remainder = 0;
        word.chars()
            .map(|n| {
                remainder = (remainder * 10 + (n.to_digit(10).unwrap() as i64)) % m as i64;
                match remainder {
                    0 => 1,
                    _ => 0,
                }
            })
            .collect()
    }

    /**
     * 2834. 找出美丽数组的最小和
     * 给你两个正整数：n 和 target 。
     * 如果数组 nums 满足下述条件，则称其为 美丽数组 。
     *     nums.length == n.
     *     nums 由两两互不相同的正整数组成。
     *     在范围 [0, n-1] 内，不存在 两个 不同 下标 i 和 j ，使得 nums[i] + nums[j] == target 。
     * 返回符合条件的美丽数组所可能具备的 最小 和，并对结果进行取模 109 + 7。
     */
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        let mod_ = 1_000_000_007;
        let mid = (target / 2) as i64;
        if n as i64 <= mid {
            let sum_less = (1 + n as i64) * n as i64 / 2;
            (sum_less % mod_) as i32
        } else {
            let sum_less = (1 + mid) * mid / 2;
            let sum_more =
                (n as i64 - mid) * (n as i64 - mid - 1) / 2 + (n as i64 - mid) * target as i64;
            ((sum_less + sum_more) % mod_) as i32
        }
    }

    /**
     * 2386. 找出数组的第 K 大和
     * 给你一个整数数组 nums 和一个 正 整数 k 。你可以选择数组的任一 子序列 并且对其全部元素求和。
     * 数组的 第 k 大和 定义为：可以获得的第 k 个 最大 子序列和（子序列和允许出现重复）
     * 返回数组的 第 k 大和 。
     * 子序列是一个可以由其他数组删除某些或不删除元素排生而来的数组，且派生过程不改变剩余元素的顺序。
     * 注意：空子序列的和视作 0 。
     */
    pub fn k_sum(mut nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;
        let mut sum = 0;
        for n in nums.iter_mut() {
            if *n < 0 {
                *n = (*n).abs();
            } else {
                sum += *n as i64;
            }
        }
        nums.sort_unstable();
        let mut heap = BinaryHeap::new();
        // 最大和
        heap.push((sum, 0));
        for _ in 1..k {
            let (curr_sum, i) = heap.pop().unwrap();
            if i < nums.len() {
                heap.push((curr_sum - nums[i] as i64, i + 1));
                if i > 0 {
                    heap.push((curr_sum + nums[i - 1] as i64 - nums[i] as i64, i + 1));
                }
            }
        }
        heap.pop().unwrap().0
    }

    /**
     * 299. 猜数字游戏
     * 你在和朋友一起玩 猜数字（Bulls and Cows）游戏，该游戏规则如下：
     * 写出一个秘密数字，并请朋友猜这个数字是多少。朋友每猜测一次，你就会给他一个包含下述信息的提示：
     *     猜测数字中有多少位属于数字和确切位置都猜对了（称为 "Bulls"，公牛），
     *     有多少位属于数字猜对了但是位置不对（称为 "Cows"，奶牛）。也就是说，这次猜测中有多少位非公牛数字可以通过重新排列转换成公牛数字。
     * 给你一个秘密数字 secret 和朋友猜测的数字 guess ，请你返回对朋友这次猜测的提示。
     * 提示的格式为 "xAyB" ，x 是公牛个数， y 是奶牛个数，A 表示公牛，B 表示奶牛。
     * 请注意秘密数字和朋友猜测的数字都可能含有重复数字。
     */
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut hash = [0; 10];
        let secret = secret.as_bytes();
        let guess = guess.as_bytes();
        let mut a = 0;
        for i in 0..secret.len() {
            if secret[i] == guess[i] {
                a += 1;
            }
            hash[(secret[i] - '0' as u8) as usize] += 1;
        }
        let mut b = 0;
        for i in 0..guess.len() {
            if hash[(guess[i] - '0' as u8) as usize] > 0 {
                hash[(guess[i] - '0' as u8) as usize] -= 1;
                b += 1;
            }
        }
        format!("{a}A{}B", b - a)
    }

    /**
     * 2789. 合并后数组中的最大元素
     * 给你一个下标从 0 开始、由正整数组成的数组 nums 。
     * 你可以在数组上执行下述操作 任意 次：
     *     选中一个同时满足 0 <= i < nums.length - 1 和 nums[i] <= nums[i + 1] 的整数 i 。
     *     将元素 nums[i + 1] 替换为 nums[i] + nums[i + 1] ，并从数组中删除元素 nums[i] 。
     * 返回你可以从最终数组中获得的 最大 元素的值。
     */
    pub fn max_array_value(mut nums: Vec<i32>) -> i64 {
        let mut ret = 0;
        for n in nums.iter().rev() {
            ret = match ret >= *n as i64 {
                true => ret + *n as i64,
                false => *n as i64,
            }
        }
        ret
    }

    /**
     * 2312. 卖木头块
     * 给你两个整数 m 和 n ，分别表示一块矩形木块的高和宽。
     * 同时给你一个二维整数数组 prices ，其中 prices[i] = [hi, wi, pricei] 表示你可以以 pricei 元的价格卖一块高为 hi 宽为 wi 的矩形木块。
     * 每一次操作中，你必须按下述方式之一执行切割操作，以得到两块更小的矩形木块：
     *     沿垂直方向按高度 完全 切割木块，或
     *     沿水平方向按宽度 完全 切割木块
     * 在将一块木块切成若干小木块后，你可以根据 prices 卖木块。你可以卖多块同样尺寸的木块。你不需要将所有小木块都卖出去。你 不能 旋转切好后木块的高和宽。
     * 请你返回切割一块大小为 m x n 的木块后，能得到的 最多 钱数。
     * 注意你可以切割木块任意次。
     */
    pub fn selling_wood(m: i32, n: i32, mut prices: Vec<Vec<i32>>) -> i64 {
        let mut p = vec![vec![0; (n + 1) as usize]; (m + 1) as usize];
        for price in prices {
            p[price[0] as usize][price[1] as usize] = price[2] as i64;
        }
        for i in 1..=m as usize {
            for j in 1..=n as usize {
                for k in 1..j {
                    p[i][j] = p[i][j].max(p[i][k] + p[i][j - k]);
                }
                for k in 1..i {
                    p[i][j] = p[i][j].max(p[k][j] + p[i - k][j]);
                }
            }
        }
        p[m as usize][n as usize]
    }

    /**
     * 2684. 矩阵中移动的最大次数
     * 给你一个下标从 0 开始、大小为 m x n 的矩阵 grid ，矩阵由若干 正 整数组成。
     * 你可以从矩阵第一列中的 任一 单元格出发，按以下方式遍历 grid ：
     *     从单元格 (row, col) 可以移动到 (row - 1, col + 1)、(row, col + 1) 和 (row + 1, col + 1) 三个单元格中任一满足值 严格 大于当前单元格的单元格。
     * 返回你在矩阵中能够 移动 的 最大 次数。
     */
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut memory = vec![vec![-1; grid[0].len()]; grid.len()];
        fn dp(grid: &Vec<Vec<i32>>, row: usize, col: usize, memory: &mut Vec<Vec<i32>>) -> i32 {
            if memory[row][col] != -1 {
                return memory[row][col];
            }
            let mut curr_res = 0;
            if col + 1 < grid[0].len() {
                if row > 0 && grid[row - 1][col + 1] > grid[row][col] {
                    curr_res = curr_res.max(1 + dp(grid, row - 1, col + 1, memory));
                }
                if grid[row][col + 1] > grid[row][col] {
                    curr_res = curr_res.max(1 + dp(grid, row, col + 1, memory));
                }
                if row + 1 < grid.len() && grid[row + 1][col + 1] > grid[row][col] {
                    curr_res = curr_res.max(1 + dp(grid, row + 1, col + 1, memory));
                }
            }
            memory[row][col] = curr_res;
            curr_res
        }
        let mut ret = 0;
        for i in 0..grid.len() {
            ret = ret.max(dp(&grid, i, 0, &mut memory));
        }
        ret
    }

    /**
     * 310. 最小高度树
     * 树是一个无向图，其中任何两个顶点只通过一条路径连接。 换句话说，一个任何没有简单环路的连通图都是一棵树。
     * 给你一棵包含 n 个节点的树，标记为 0 到 n - 1 。
     * 给定数字 n 和一个有 n - 1 条无向边的 edges 列表（每一个边都是一对标签），
     * 其中 edges[i] = [ai, bi] 表示树中节点 ai 和 bi 之间存在一条无向边。
     * 可选择树中任何一个节点作为根。当选择节点 x 作为根节点时，设结果树的高度为 h 。
     * 在所有可能的树中，具有最小高度的树（即，min(h)）被称为 最小高度树 。
     * 请你找到所有的 最小高度树 并按 任意顺序 返回它们的根节点标签列表。
     * 树的 高度 是指根节点和叶子节点之间最长向下路径上边的数量。
     */
    pub fn find_min_height_trees(n: i32, mut edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut g = vec![vec![]; n as usize];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        fn dfs(g: &Vec<Vec<i32>>, node: i32, from: i32) -> (i32, i32) {
            let mut max_distance = 0;
            let mut max_node = node;
            for &next in &g[node as usize] {
                if next != from {
                    let (m, n) = dfs(g, next, node);
                    if m + 1 > max_distance {
                        max_distance = m + 1;
                        max_node = n;
                    }
                }
            }
            (max_distance, max_node)
        }
        let x = dfs(&g, 0, 0).1;
        let y = dfs(&g, x, x).1;
        let mut way = vec![];
        fn find_way(
            g: &Vec<Vec<i32>>,
            node: i32,
            from: i32,
            target: i32,
            way: &mut Vec<i32>,
        ) -> bool {
            if node == target {
                way.push(node);
                return true;
            }
            for &next in &g[node as usize] {
                if next != from && find_way(g, next, node, target, way) {
                    way.push(node);
                    return true;
                }
            }
            false
        }
        find_way(&g, x, x, y, &mut way);
        match way.len() % 2 == 0 {
            true => vec![way[way.len() / 2 - 1], way[way.len() / 2]],
            false => vec![way[way.len() / 2]],
        }
    }
}
