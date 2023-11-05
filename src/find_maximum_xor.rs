use std::collections::HashSet;

pub struct Solution {}
/**
 * 421. 数组中两个数的最大异或值
 * 给你一个整数数组 nums ，返回 nums[i] XOR nums[j] 的最大运算结果，其中 0 ≤ i ≤ j < n 。
 */
impl Solution {
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
}
