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
}
