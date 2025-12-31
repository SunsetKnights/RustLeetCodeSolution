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
    /**
     * 2845. 统计趣味子数组的数目
     * 给你一个下标从 0 开始的整数数组 nums ，以及整数 modulo 和整数 k 。
     * 请你找出并统计数组中 趣味子数组 的数目。
     * 如果 子数组 nums[l..r] 满足下述条件，则称其为 趣味子数组 ：
     *     在范围 [l, r] 内，设 cnt 为满足 nums[i] % modulo == k 的索引 i 的数量。并且 cnt % modulo == k 。
     * 以整数形式表示并返回趣味子数组的数目。
     * 注意：子数组是数组中的一个连续非空的元素序列。
     */
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let mut count = 0;
        let mut result = 0;
        let mut map = std::collections::HashMap::new();
        map.insert(0, 1);
        for i in nums {
            if i % modulo == k {
                count += 1;
            }
            let key = (count - k) % modulo;
            if let Some(&value) = map.get(&key) {
                result += value;
            }
            *map.entry(count % modulo).or_insert(0) += 1;
        }
        result
    }
    /**
     * 2444. 统计定界子数组的数目
     * 给你一个整数数组 nums 和两个整数 minK 以及 maxK 。
     * nums 的定界子数组是满足下述条件的一个子数组：
     *     子数组中的 最小值 等于 minK 。
     *     子数组中的 最大值 等于 maxK 。
     * 返回定界子数组的数目。
     * 子数组是数组中的一个连续部分。
     */
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let n = nums.len();
        let mut result = 0;
        let mut min_idx = None;
        let mut max_idx = None;
        let mut start = 0;
        for i in 0..n {
            // 过界，更新下一段的起始点
            if nums[i] < min_k || nums[i] > max_k {
                start = i + 1;
                min_idx = None;
                max_idx = None;
            } else {
                if nums[i] == min_k {
                    min_idx = Some(i);
                }
                if nums[i] == max_k {
                    max_idx = Some(i);
                }
                // 最大最小值已找到且没有过界，枚举结束点
                if let (Some(min), Some(max)) = (min_idx.clone(), max_idx.clone()) {
                    result += (min.min(max) - start + 1) as i64;
                }
            }
        }
        result
    }
    /**
     * 2962. 统计最大元素出现至少 K 次的子数组
     * 给你一个整数数组 nums 和一个 正整数 k 。
     * 请你统计有多少满足 「 nums 中的 最大 元素」至少出现 k 次的子数组，并返回满足这一条件的子数组的数目。
     * 子数组是数组中的一个连续元素序列。
     */
    pub fn count_subarrays_2962(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut locations = Vec::new();
        let mut result = 0;
        let mut max_num = 0;
        nums.iter().enumerate().for_each(|(i, &n)| {
            if n > max_num {
                max_num = n;
                locations.clear();
                locations.push(i);
            }
            if n == max_num {
                locations.push(i);
            }
        });
        if locations.len() < k as usize {
            return 0;
        }
        // 枚举起始点
        let mut left = 0;
        while left + k as usize - 1 < locations.len() {
            let right = locations[left + k as usize - 1];
            let left_bound = if left == 0 {
                locations[left] + 1
            } else {
                locations[left] - locations[left - 1]
            };
            result += left_bound as i64 * (n - right) as i64;
            left += 1;
        }
        result
    }
}
