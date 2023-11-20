pub struct Solution {}
/**
 * 53. 最大子数组和
 * 给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
 * 子数组 是数组中的一个连续部分。
 */
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut curr_max = 0;
        for num in nums {
            curr_max = num.max(num + curr_max);
            max = max.max(curr_max);
        }
        max
    }
}
