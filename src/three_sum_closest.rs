pub struct Solution {}
/**
 * 16. 最接近的三数之和
 * 给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。
 * 返回这三个数的和。
 * 假定每组输入只存在恰好一个解。
 */
impl Solution {
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
}
