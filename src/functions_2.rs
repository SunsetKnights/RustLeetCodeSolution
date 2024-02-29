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
}
