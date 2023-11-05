use std::collections::HashMap;

pub struct Solution {}
/**
 *给你一个由 不同 正整数组成的数组 nums ，
 *请你返回满足 a * b = c * d 的元组 (a, b, c, d) 的数量。
 *其中 a、b、c 和 d 都是 nums 中的元素，且 a != b != c != d 。
 */
impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
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
                result += Self::get_c(*v, 2) * 8;
            }
        }
        result
    }
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
}
