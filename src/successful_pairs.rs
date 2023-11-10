pub struct Solution {}
/**
 * 2300. 咒语和药水的成功对数
 * 给你两个正整数数组 spells 和 potions ，长度分别为 n 和 m ，其中 spells[i] 表示第 i 个咒语的能量强度，potions[j] 表示第 j 瓶药水的能量强度。
 * 同时给你一个整数 success 。一个咒语和药水的能量强度 相乘 如果 大于等于 success ，那么它们视为一对 成功 的组合。
 * 请你返回一个长度为 n 的整数数组 pairs，其中 pairs[i] 是能跟第 i 个咒语成功组合的 药水 数目。
 */
impl Solution {
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
}
