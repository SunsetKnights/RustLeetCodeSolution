pub struct Solution {}
/**
 * 689. 三个无重叠子数组的最大和
 * 给你一个整数数组 nums 和一个整数 k ，找出三个长度为 k 、互不重叠、且全部数字和（3 * k 项）最大的子数组，并返回这三个子数组。
 * 以下标的数组形式返回结果，数组中的每一项分别指示每个子数组的起始位置（下标从 0 开始）。如果有多个结果，返回字典序最小的一个。
 */
impl Solution {
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
}
