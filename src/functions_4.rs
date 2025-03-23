pub struct Solution;
#[allow(unused)]
impl Solution {
    /**
     * 41. 缺失的第一个正数
     * 给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。
     * 请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。
     */
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            // 把值为n的元素放到数组下标n-1的位置
            while nums[i] > 0 && nums[i] as usize <= n && nums[i] as usize - 1 != i {
                let temp = nums[i] as usize - 1;
                if nums[temp] == nums[i] {
                    nums[temp] = -1;
                }
                nums.swap(i, temp);
            }
            if nums[i] != (i + 1) as i32 {
                nums[i] = -1;
            }
        }
        nums.iter()
            .enumerate()
            .find(|(idx, &x)| idx + 1 != x as usize)
            .map(|(idx, _)| idx as i32 + 1)
            .or(Some(n as i32 + 1))
            .unwrap()
    }
    /**
     * 2610. 转换二维数组
     * 给你一个整数数组 nums 。请你创建一个满足以下条件的二维数组：
     *     二维数组应该 只 包含数组 nums 中的元素。
     *     二维数组中的每一行都包含 不同 的整数。
     *     二维数组的行数应尽可能 少 。
     * 返回结果数组。如果存在多种答案，则返回其中任何一种。
     * 请注意，二维数组的每一行上可以存在不同数量的元素。
     */
    pub fn find_matrix(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result = vec![vec![]];
        let mut nums_idx = 0;
        let mut result_idx = 0;
        while nums_idx < nums.len() {
            result[0].push(nums[nums_idx]);
            nums_idx += 1;
            result_idx = 1;
            while nums_idx < nums.len() && nums[nums_idx] == nums[nums_idx - 1] {
                if result.len() <= result_idx {
                    result.push(Vec::new());
                }
                result[result_idx].push(nums[nums_idx]);
                nums_idx += 1;
                result_idx += 1;
            }
        }
        result
    }
}
