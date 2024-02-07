use std::collections::HashSet;

pub struct Solution;
impl Solution {
    /**
     * 421. 数组中两个数的最大异或值
     * 给你一个整数数组 nums ，返回 nums[i] XOR nums[j] 的最大运算结果，其中 0 ≤ i ≤ j < n 。
     */
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        //一定存在一个最大值，假设这个最大值的每一位都是1
        //从高位到低位找所有数字的前n(1<=n<=30)位
        //从第30位开始，若这一位是1，则必能找到两个num的前缀异或为1
        //到第29位，此时已经知道第30位是1还是0
        let mut result = 0;
        let mut front_maker = 0;
        let mut all_front: HashSet<i32> = HashSet::new();
        for i in (0..32).rev() {
            front_maker |= 1 << i;
            for n in nums.iter() {
                all_front.insert(*n & front_maker);
            }
            for n in nums.iter() {
                if all_front.contains(&((*n & front_maker) ^ (result | 1 << i))) {
                    result |= 1 << i;
                    break;
                }
            }
            all_front.clear();
        }
        result
    }

    /**
     * 187. 重复的DNA序列
     * DNA序列 由一系列核苷酸组成，缩写为 'A', 'C', 'G' 和 'T'。
     * 例如，"ACGAATTCCG" 是一个 DNA序列 。
     * 在研究 DNA 时，识别 DNA 中的重复序列非常有用。
     * 给定一个表示 DNA序列 的字符串 s ，返回所有在 DNA 分子中出现不止一次的 长度为 10 的序列(子字符串)。你可以按 任意顺序 返回答案。
     */
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        //把每一个可能的十位序列存入一个hashmap
        //把map中v>=2的加入结果集（这一步跟第一步可以在同一循环中实现）
        //由于只出现4个字母，可以压缩一下存储空间，用0123的二进制码分别代表ACGT，这样每个十位序列只需要一个i32存储
        let s = s.as_bytes();
        let mut counter: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<String> = Vec::new();
        let mut key: i32;
        for i in 10..s.len() + 1 {
            key = 0;
            for c in i - 10..i {
                key |= match s[c] {
                    b'A' => 0,
                    b'C' => 1,
                    b'G' => 2,
                    b'T' => 3,
                    _ => 0,
                } << (i - c - 1) * 2;
            }
            let count = counter.entry(key).or_insert(0);
            *count += 1;
            if *count == 2 {
                result.push(std::str::from_utf8(&s[i - 10..i]).unwrap().to_string());
            }
        }
        result
    }

    /**
     * 1334. 阈值距离内邻居最少的城市
     * 有 n 个城市，按从 0 到 n-1 编号。给你一个边数组 edges，其中 edges[i] = [fromi, toi, weighti] 代表 fromi 和 toi 两个城市之间的双向加权边，距离阈值是一个整数 distanceThreshold。
     * 返回能通过某些路径到达其他城市数目最少、且路径距离 最大 为 distanceThreshold 的城市。如果有多个这样的城市，则返回编号最大的城市。
     * 注意，连接城市 i 和 j 的路径的距离等于沿该路径的所有边的权重之和。
     */
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

    /**
     * 2661. 找出叠涂元素
     * 给你一个下标从 0 开始的整数数组 arr 和一个 m x n 的整数 矩阵 mat 。arr 和 mat 都包含范围 [1，m * n] 内的 所有 整数。
     * 从下标 0 开始遍历 arr 中的每个下标 i ，并将包含整数 arr[i] 的 mat 单元格涂色。
     * 请你找出 arr 中在 mat 的某一行或某一列上都被涂色且下标最小的元素，并返回其下标 i 。
     */
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        if arr.len() == 1 {
            return 0;
        }
        let row_count = mat.len();
        let col_count = mat[0].len();
        let mut element_local: HashMap<i32, (usize, usize)> = HashMap::new();
        for (row_i, row) in mat.iter().enumerate() {
            for (col_i, val) in row.iter().enumerate() {
                element_local.insert(*val, (row_i, col_i));
            }
        }
        let mut rows = vec![col_count; row_count];
        let mut cols = vec![row_count; col_count];
        for (i, val) in arr.iter().enumerate() {
            let (row_i, col_i) = element_local[val];
            rows[row_i] -= 1;
            cols[col_i] -= 1;
            if rows[row_i] == 0 || cols[col_i] == 0 {
                return i as i32;
            }
        }
        -1
    }

    /**
     * 22. 括号生成
     * 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
     */
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut list: Vec<Vec<String>> = vec![Vec::new(); n + 1];
        list[0].push(String::from(""));
        list[1].push(String::from("()"));
        for i in 2..=n {
            for j in 0..i {
                for s_in_i in 0..list[j].len() {
                    let s_in = list[j][s_in_i].clone();
                    for s_right_i in 0..list[i - 1 - j].len() {
                        let s_right = list[i - 1 - j][s_right_i].clone();
                        list[i].push(format!("({}){}", s_in, s_right));
                    }
                }
            }
        }
        list[n].clone()
    }

    /**
     * 给你一个整数数组 citations ，其中 citations[i] 表示研究者的第 i 篇论文被引用的次数。计算并返回该研究者的 h 指数。
     * 根据维基百科上 h 指数的定义：h 代表“高引用次数” ，一名科研人员的 h 指数 是指他（她）至少发表了 h 篇论文，
     * 并且每篇论文 至少 被引用 h 次。如果 h 有多种可能的值，h 指数 是其中最大的那个。
     */
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut sortable = citations.clone();
        sortable.sort_unstable();
        let size: i32 = sortable.len() as i32;
        let mut low: i32 = 0;
        let mut high: i32 = size - 1;
        let mut mid: i32 = (low + high) / 2;
        while low <= high {
            if sortable[mid as usize] < (size - mid) {
                low = mid + 1;
            } else if sortable[mid as usize] > (size - mid) {
                high = mid - 1;
            } else {
                return size - mid;
            }
            mid = (low + high) / 2;
        }
        size - low
    }

    /**
     * 36. 有效的数独
     * 请你判断一个 9 x 9 的数独是否有效。只需要 根据以下规则 ，验证已经填入的数字是否有效即可。
     *     数字 1-9 在每一行只能出现一次。
     *     数字 1-9 在每一列只能出现一次。
     *     数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。（请参考示例图）
     */
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut matrix: HashSet<usize> = HashSet::new();
        let mut g;
        let mut v;
        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    continue;
                }
                g = (r / 3) * 3 + c / 3;
                v = board[r][c].to_string().parse::<usize>().unwrap();
                if matrix.contains(&(r * 9 + v - 1)) {
                    return false;
                }
                matrix.insert(r * 9 + v - 1);
                if matrix.contains(&(81 + c * 9 + v - 1)) {
                    return false;
                }
                matrix.insert(81 + c * 9 + v - 1);
                if matrix.contains(&(162 + g * 9 + v - 1)) {
                    return false;
                }
                matrix.insert(162 + g * 9 + v - 1);
            }
        }
        true
    }

    /**
     * 318. 最大单词长度乘积
     * 给你一个字符串数组 words ，找出并返回 length(words[i]) * length(words[j]) 的最大值，
     * 并且这两个单词不含有公共字母。如果不存在这样的两个单词，返回 0 。
     */
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut temp;
        let mut result = 0;
        for w in words {
            temp = 0;
            for c in w.as_bytes() {
                temp |= 1 << *c - 97;
            }
            let count = map.entry(temp).or_insert(0);
            *count = if *count > w.len() as i32 {
                *count
            } else {
                w.len() as i32
            };
        }
        let keys: Vec<&i32> = map.keys().clone().collect();
        for i in 0..map.len() - 1 {
            for j in i + 1..map.len() {
                if keys[i] & keys[j] == 0 && map[keys[i]] * map[keys[j]] > result {
                    result = map[keys[i]] * map[keys[j]];
                }
            }
        }
        result
    }

    /**
     * 1349. 参加考试的最大学生数
     * 给你一个 m * n 的矩阵 seats 表示教室中的座位分布。如果座位是坏的（不可用），就用 '#' 表示；否则，用 '.' 表示。
     * 学生可以看到左侧、右侧、左上、右上这四个方向上紧邻他的学生的答卷，
     * 但是看不到直接坐在他前面或者后面的学生的答卷。
     * 请你计算并返回该考场可以容纳的同时参加考试且无法作弊的最大学生人数。
     * 学生必须坐在状况良好的座位上。
     */
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let row_quantity = seats.len();
        let col_quantity = seats[0].len();
        let row_val_max = ((1usize << col_quantity) - 1) as u8;
        // Store seats as bitmap
        let mut seats_bit = vec![0u8; row_quantity];
        for (i, row) in seats.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == '#' {
                    seats_bit[i] |= 1 << j;
                }
            }
        }
        fn check(curr_row: u8, upper_row: u8, curr_seats: u8) -> bool {
            curr_row & curr_seats == 0
                && curr_row & (curr_row >> 1) == 0
                && (curr_row << 1) & upper_row == 0
                && (curr_row >> 1) & upper_row == 0
                && curr_row & (curr_row << 1) == 0
        }
        fn count(row: u8) -> i32 {
            let mut row = row;
            let mut ret = 0;
            while row != 0 {
                row &= row - 1;
                ret += 1;
            }
            ret
        }
        // Vec<(u8, i32)>
        // u8: Current student seating situation.
        // i32: Maximum number of students.
        let mut upper_rows: Vec<(u8, i32)> = Vec::new();
        for curr_stu in 0..row_val_max {
            if check(curr_stu, 0, seats_bit[0]) {
                upper_rows.push((curr_stu, count(curr_stu)));
            }
        }
        for i in 1..row_quantity {
            let mut curr_rows = Vec::new();
            for curr_situation in 0..=row_val_max {
                let mut curr_max = 0;
                for &(upper_stu, quantity) in &upper_rows {
                    if check(curr_situation, upper_stu, seats_bit[i]) {
                        curr_max = curr_max.max(count(curr_situation) + quantity);
                    }
                }
                curr_rows.push((curr_situation, curr_max));
            }
            upper_rows = curr_rows;
        }
        let mut ret = 0;
        for &(_, quantity) in &upper_rows {
            ret = ret.max(quantity);
        }
        ret
    }

    /**
     * 53. 最大子数组和
     * 给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
     * 子数组 是数组中的一个连续部分。
     */
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut curr_max = 0;
        for num in nums {
            curr_max = num.max(num + curr_max);
            max = max.max(curr_max);
        }
        max
    }

    /**
     * 689. 三个无重叠子数组的最大和
     * 给你一个整数数组 nums 和一个整数 k ，找出三个长度为 k 、互不重叠、且全部数字和（3 * k 项）最大的子数组，并返回这三个子数组。
     * 以下标的数组形式返回结果，数组中的每一项分别指示每个子数组的起始位置（下标从 0 开始）。如果有多个结果，返回字典序最小的一个。
     */
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut nums = nums;
        nums.reverse();
        let size = nums.len();
        let mut prefix_sum: Vec<i64> = vec![0; size + 1];
        prefix_sum[1] = nums[0] as i64;
        // 为方便统一处理，前缀和跟最大和矩阵的下标都从1开始
        for i in 1..=size {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1] as i64;
        }
        let mut f_range: Vec<Vec<i64>> = vec![vec![0; 4]; size + 1];
        let mut temp;
        for i in k..=size {
            for j in 1..4 {
                temp = f_range[i - k][j - 1] + prefix_sum[i] - prefix_sum[i - k];
                f_range[i][j] = if f_range[i - 1][j] > temp {
                    f_range[i - 1][j]
                } else {
                    temp
                }
            }
        }

        let mut ret = vec![0; 3];
        let mut i = size;
        let mut j = 3;
        let mut ret_i = 0;
        while j > 0 {
            if f_range[i - 1][j] > f_range[i - k][j - 1] + prefix_sum[i] - prefix_sum[i - k] {
                i -= 1;
            } else {
                ret[ret_i] = (size - i) as i32;
                ret_i += 1;
                j -= 1;
                i -= k;
            }
        }
        ret
    }

    /**
     * 2008. 出租车的最大盈利
     * 你驾驶出租车行驶在一条有 n 个地点的路上。这 n 个地点从近到远编号为 1 到 n ，你想要从 1 开到 n ，通过接乘客订单盈利。你只能沿着编号递增的方向前进，不能改变方向。
     * 乘客信息用一个下标从 0 开始的二维数组 rides 表示，其中 rides[i] = [starti, endi, tipi] 表示第 i 位乘客需要从地点 starti 前往 endi ，愿意支付 tipi 元的小费。
     * 每一位 你选择接单的乘客 i ，你可以 盈利 endi - starti + tipi 元。你同时 最多 只能接一个订单。
     * 给你 n 和 rides ，请你返回在最优接单方案下，你能盈利 最多 多少元。
     * 注意：你可以在一个地点放下一位乘客，并在同一个地点接上另一位乘客。
     */
    pub fn max_taxi_earnings(_n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let mut max = vec![0i64; rides.len() + 1];
        let mut rides: Vec<(i32, i32, i32)> = rides.iter().map(|x| (x[1], x[0], x[2])).collect();
        rides.sort_unstable();
        for i in 1..=rides.len() {
            let curr = &rides[i - 1];
            let front = rides.partition_point(|&p| p.0 <= curr.1);
            let include_curr = (curr.0 - curr.1 + curr.2) as i64 + max[front];
            let not_include_curr = max[i - 1];
            max[i] = include_curr.max(not_include_curr);
        }
        max[rides.len()]
    }

    /**
     * 2127. 参加会议的最多员工数
     * 一个公司准备组织一场会议，邀请名单上有 n 位员工。公司准备了一张 圆形 的桌子，可以坐下 任意数目 的员工。
     * 员工编号为 0 到 n - 1 。每位员工都有一位 喜欢 的员工，每位员工 当且仅当 他被安排在喜欢员工的旁边，他才会参加会议。每位员工喜欢的员工不会是他自己。
     * 给你一个下标从 0 开始的整数数组 favorite ，其中 favorite[i] 表示第 i 位员工喜欢的员工。请你返回参加会议的 最多员工数目 。
     */
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
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

    /**
     * 2258. 逃离火灾
     * 给你一个下标从 0 开始大小为 m x n 的二维整数数组 grid ，它表示一个网格图。每个格子为下面 3 个值之一：
     * 0 表示草地。
     * 1 表示着火的格子。
     * 2 表示一座墙，你跟火都不能通过这个格子。
     * 一开始你在最左上角的格子 (0, 0) ，你想要到达最右下角的安全屋格子 (m - 1, n - 1) 。每一分钟，你可以移动到 相邻 的草地格子。
     * 每次你移动 之后 ，着火的格子会扩散到所有不是墙的 相邻 格子。
     * 请你返回你在初始位置可以停留的 最多 分钟数，且停留完这段时间后你还能安全到达安全屋。如果无法实现，请你返回 -1 。
     * 如果不管你在初始位置停留多久，你 总是 能到达安全屋，请你返回 10^9 。
     * 注意，如果你到达安全屋后，火马上到了安全屋，这视为你能够安全到达安全屋。
     * 如果两个格子有共同边，那么它们为 相邻 格子。
     */
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        //求出location中的格子到右下角，右下角的左边，右下角的上面的最短时间
        fn min_time(mut location: Vec<(i32, i32)>, grid: &Vec<Vec<i32>>) -> (i32, i32, i32) {
            let row = grid.len();
            let col = grid[0].len();
            let mut result = (0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF);
            let mut times = vec![vec![0x7FFFFFFF; col]; row];
            for p in location.iter() {
                times[p.0 as usize][p.1 as usize] = 0;
            }
            let mut time = 1;
            while !location.is_empty() {
                let mut layer = Vec::new();
                for p in location {
                    for (i, j) in [
                        (p.0 + 1, p.1),
                        (p.0 - 1, p.1),
                        (p.0, p.1 + 1),
                        (p.0, p.1 - 1),
                    ] {
                        if i >= 0
                            && (i < row as i32)
                            && j >= 0
                            && (j < col as i32)
                            && grid[i as usize][j as usize] != 2
                            && times[i as usize][j as usize] == 0x7FFFFFFF
                        {
                            times[i as usize][j as usize] = time;
                            layer.push((i, j));
                        }
                    }
                }
                time += 1;
                location = layer;
            }
            result.0 = times[row - 1][col - 1];
            result.1 = times[row - 1][col - 2];
            result.2 = times[row - 2][col - 1];
            result
        }
        let person_time = Self::min_time(vec![(0, 0)], &grid);
        let mut fire_locations = Vec::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    fire_locations.push((i as i32, j as i32));
                }
            }
        }
        let fire_time = Self::min_time(fire_locations, &grid);
        let distance = fire_time.0 - person_time.0;
        if person_time.0 == 0x7FFFFFFF || distance < 0 {
            return -1;
        } else if fire_time.0 == 0x7FFFFFFF {
            return 1000000000;
        } else {
            if distance < fire_time.1 - person_time.1 || distance < fire_time.2 - person_time.2 {
                return distance;
            }
            return distance - 1;
        }
    }

    /**
     * 2736. 最大和查询
     * 给你两个长度为 n 、下标从 0 开始的整数数组 nums1 和 nums2 ，另给你一个下标从 1 开始的二维数组 queries ，其中 queries[i] = [xi, yi] 。
     * 对于第 i 个查询，在所有满足 nums1[j] >= xi 且 nums2[j] >= yi 的下标 j (0 <= j < n) 中，找出 nums1[j] + nums2[j] 的 最大值 ，
     * 如果不存在满足条件的 j 则返回 -1 。
     * 返回数组 answer ，其中 answer[i] 是第 i 个查询的答案。
     */
    pub fn maximum_sum_queries(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        // 把两个数组看成一个平面上点的数组，按照x值降序排列
        let mut a: Vec<(i32, i32)> = nums1.into_iter().zip(nums2.into_iter()).collect();
        a.sort_by(|x, y| y.0.cmp(&x.0));

        // 把待查询的点按照x值降序排列，查询范围即是待查询点的右上部分
        let mut queries_index: Vec<usize> = (0..queries.len()).collect();
        queries_index.sort_by(|&i, &j| queries[j][0].cmp(&queries[i][0]));

        let mut ret = vec![-1; queries.len()];
        let mut sorted_stack: Vec<(i32, i32)> = Vec::new();
        let mut j = 0;
        for &i in &queries_index {
            let x = queries[i][0];
            let y = queries[i][1];
            // 遍历待查询点右边的点
            while j < a.len() && a[j].0 >= x {
                // 若当前点的y值小于栈顶的y，跳过
                // 若当前点的y值大于栈顶的y，后续可能有用，要入栈，入栈前可以把i小于当前点的x+y的点弹出
                // 由于x越来越小，所以如果x+y大于栈顶元素，那么y一定大于栈顶元素，只需要保留y更大的点
                while !sorted_stack.is_empty() && sorted_stack.last().unwrap().1 <= a[j].0 + a[j].1
                {
                    sorted_stack.pop();
                }

                if sorted_stack.is_empty() || sorted_stack.last().unwrap().0 < a[j].1 {
                    sorted_stack.push((a[j].1, a[j].0 + a[j].1));
                }
                j += 1;
            }
            let p = sorted_stack.partition_point(|&p| p.0 < y);
            if p < sorted_stack.len() {
                ret[i] = sorted_stack[p].1;
            }
        }
        ret
    }

    /**
     * 2342. 数位和相等数对的最大和
     * 给你一个下标从 0 开始的数组 nums ，数组中的元素都是 正 整数。
     * 请你选出两个下标 i 和 j（i != j），且 nums[i] 的数位和 与  nums[j] 的数位和相等。
     * 请你找出所有满足条件的下标 i 和 j ，找出并返回 nums[i] + nums[j] 可以得到的 最大值 。
     */
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut sum_digits: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..nums.len() {
            let mut s = 0;
            let mut temp = nums[i];
            while temp > 0 {
                s += temp % 10;
                temp /= 10;
            }
            let n = sum_digits.entry(s).or_insert(Vec::new());
            let idx = n.partition_point(|&x| x < nums[i]);
            n.insert(idx, nums[i]);
        }
        let mut ret = -1;
        for v in sum_digits.values() {
            if v.len() <= 1 {
                continue;
            } else {
                let max_sum = v[v.len() - 1] + v[v.len() - 2];
                ret = if ret < max_sum { max_sum } else { ret }
            }
        }
        ret
    }

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

    /**
     * 1631. 最小体力消耗路径
     * 你准备参加一场远足活动。给你一个二维 rows x columns 的地图 heights ，其中 heights[row][col] 表示格子 (row, col) 的高度。
     * 一开始你在最左上角的格子 (0, 0) ，且你希望去最右下角的格子 (rows-1, columns-1) （注意下标从 0 开始编号）。
     * 你每次可以往 上，下，左，右 四个方向之一移动，你想要找到耗费 体力 最小的一条路径。
     * 一条路径耗费的 体力值 是路径上相邻格子之间 高度差绝对值 的 最大值 决定的。
     * 请你返回从左上角走到右下角的最小 体力消耗值 。
     */
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

    /**
     * 2646. 最小化旅行的价格总和
     * 现有一棵无向、无根的树，树中有 n 个节点，按从 0 到 n - 1 编号。给你一个整数 n 和一个长度为 n - 1 的二维整数数组 edges ，其中 edges[i] = [ai, bi] 表示树中节点 ai 和 bi 之间存在一条边。
     * 每个节点都关联一个价格。给你一个整数数组 price ，其中 price[i] 是第 i 个节点的价格。
     * 给定路径的 价格总和 是该路径上所有节点的价格之和。
     * 另给你一个二维整数数组 trips ，其中 trips[i] = [starti, endi] 表示您从节点 starti 开始第 i 次旅行，并通过任何你喜欢的路径前往节点 endi 。
     * 在执行第一次旅行之前，你可以选择一些 非相邻节点 并将价格减半。
     * 返回执行所有旅行的最小价格总和。
     */
    pub fn minimum_total_price(
        n: i32,
        edges: Vec<Vec<i32>>,
        price: Vec<i32>,
        trips: Vec<Vec<i32>>,
    ) -> i32 {
        let n = n as usize;
        let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); n];
        for i in edges {
            let s = i[0] as usize;
            let e = i[1] as usize;
            adj_list[s].push(e);
            adj_list[e].push(s);
        }
        // DFS统计每个节点经过的次数
        fn dfs_counter(
            start: usize,
            end: usize,
            from: usize,
            counter: &mut Vec<i32>,
            adj_list: &Vec<Vec<usize>>,
        ) -> bool {
            if start == end {
                return true;
            }
            for &i in &adj_list[start] {
                if i != from && dfs_counter(i, end, start, counter, adj_list) {
                    counter[i] += 1;
                    return true;
                }
            }
            return false;
        }
        let mut counter = vec![0; n];
        for trip in trips {
            let start = trip[0] as usize;
            let end = trip[1] as usize;
            counter[start] += 1;
            dfs_counter(start, end, start, &mut counter, &adj_list);
        }
        // 统计节点经过次数和节点价格乘积和，把一些不相邻的节点价格/2，使和最小
        fn dfs_price(
            curr: usize,
            from: usize,
            counter: &Vec<i32>,
            price: &Vec<i32>,
            adj_list: &Vec<Vec<usize>>,
        ) -> (i32, i32) {
            let mut curr_not_half = counter[curr] * price[curr];
            let mut curr_half = curr_not_half / 2;
            for &i in &adj_list[curr] {
                if i != from {
                    let (not_half, half) = dfs_price(i, curr, counter, price, adj_list);
                    curr_half += not_half;
                    curr_not_half = (curr_not_half + not_half).min(curr_not_half + half);
                }
            }
            (curr_not_half, curr_half)
        }
        let (not_half, half) = dfs_price(0, 0, &counter, &price, &adj_list);
        not_half.min(half)
    }

    /**
     * 2132. 用邮票贴满网格图
     * 给你一个 m x n 的二进制矩阵 grid ，每个格子要么为 0 （空）要么为 1 （被占据）。
     * 给你邮票的尺寸为 stampHeight x stampWidth 。我们想将邮票贴进二进制矩阵中，且满足以下 限制 和 要求 ：
     *
     *     覆盖所有 空 格子。
     *     不覆盖任何 被占据 的格子。
     *     我们可以放入任意数目的邮票。
     *     邮票可以相互有 重叠 部分。
     *     邮票不允许 旋转 。
     *     邮票必须完全在矩阵 内 。
     *
     * 如果在满足上述要求的前提下，可以放入邮票，请返回 true ，否则返回 false 。
     */
    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        let height = grid.len();
        let width = grid[0].len();
        let stamp_height = stamp_height as usize;
        let stamp_width = stamp_width as usize;
        // 方便判断区域是否有被占据的格子，若有，区域和为1
        let mut prefix_sum = vec![vec![0; width + 2]; height + 2];
        // 方便给区域内的所有格子增加数字
        let mut diff = vec![vec![0; width + 2]; height + 2];
        for i in 1..=height {
            for j in 1..=width {
                prefix_sum[i][j] = grid[i - 1][j - 1] + prefix_sum[i - 1][j] + prefix_sum[i][j - 1]
                    - prefix_sum[i - 1][j - 1];
            }
        }

        if height >= stamp_height && width >= stamp_width {
            for i in 1..=height - stamp_height + 1 {
                for j in 1..=width - stamp_width + 1 {
                    let x = i + stamp_height - 1;
                    let y = j + stamp_width - 1;
                    // 以i,j为起始格子的邮票大小的格子区域为空
                    if prefix_sum[x][y] - prefix_sum[i - 1][y] - prefix_sum[x][j - 1]
                        + prefix_sum[i - 1][j - 1]
                        == 0
                    {
                        // 区域整体+1
                        diff[i][j] += 1;
                        diff[x + 1][y + 1] += 1;
                        diff[x + 1][j] -= 1;
                        diff[i][y + 1] -= 1;
                    }
                }
            }
        }
        for i in 1..=height {
            for j in 1..=width {
                diff[i][j] += diff[i - 1][j] + diff[i][j - 1] - diff[i - 1][j - 1];
                if diff[i][j] == 0 && grid[i - 1][j - 1] == 0 {
                    return false;
                }
            }
        }
        true
    }

    /**
     * 有一棵根节点为 0 的 家族树 ，总共包含 n 个节点，节点编号为 0 到 n - 1 。
     * 给你一个下标从 0 开始的整数数组 parents ，其中 parents[i] 是节点 i 的父节点。由于节点 0 是 根 ，所以 parents[0] == -1 。
     * 总共有 10^5 个基因值，每个基因值都用 闭区间 [1, 10^5] 中的一个整数表示。
     * 给你一个下标从 0 开始的整数数组 nums ，其中 nums[i] 是节点 i 的基因值，且基因值 互不相同 。
     * 请你返回一个数组 ans ，长度为 n ，其中 ans[i] 是以节点 i 为根的子树内 缺失 的 最小 基因值。
     * 节点 x 为根的 子树 包含节点 x 和它所有的 后代 节点。
     */
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let size = parents.len();
        let mut result = vec![1; size];
        // 不存在基因值为1的点，说明所有的节点缺失的最小基因都为1，直接返回答案
        let mut node = match nums.iter().position(|&x| x == 1) {
            Some(i) => i as i32,
            None => return result,
        };

        // 方便遍历树
        let mut tree = vec![vec![]; size];
        for i in 1..size {
            tree[parents[i] as usize].push(i as i32);
        }

        //由于最小缺失基因值不为1的节点一定在从基因值为1的节点到根节点的一条路上，所以可以把所有基因值保存在一个hash表里面判断
        let mut all_child_nums: HashSet<i32> = HashSet::new();
        let mut stack: Vec<i32> = Vec::new();
        let mut smallest = 2;
        while node >= 0 {
            stack.clear();
            stack.push(node);
            while !stack.is_empty() {
                let x = stack.pop().unwrap();
                all_child_nums.insert(nums[x as usize]);
                stack.append(&mut tree[x as usize]);
            }
            while all_child_nums.contains(&smallest) {
                smallest += 1;
            }
            result[node as usize] = smallest;
            node = parents[node as usize];
        }
        result
    }
}
