use std::collections::HashMap;

pub struct Solution {}
/**
 * 2342. 数位和相等数对的最大和
 * 给你一个下标从 0 开始的数组 nums ，数组中的元素都是 正 整数。
 * 请你选出两个下标 i 和 j（i != j），且 nums[i] 的数位和 与  nums[j] 的数位和相等。
 * 请你找出所有满足条件的下标 i 和 j ，找出并返回 nums[i] + nums[j] 可以得到的 最大值 。
 */
impl Solution {
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
}
