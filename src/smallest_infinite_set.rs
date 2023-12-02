/**
 * 2336. 无限集中的最小数字
 * 现有一个包含所有正整数的集合 [1, 2, 3, 4, 5, ...] 。
 * 实现 SmallestInfiniteSet 类：
 *     SmallestInfiniteSet() 初始化 SmallestInfiniteSet 对象以包含 所有 正整数。
 *     int popSmallest() 移除 并返回该无限集中的最小整数。
 *     void addBack(int num) 如果正整数 num 不 存在于无限集中，则将一个 num 添加 到该无限集最后。
 */

pub struct SmallestInfiniteSet {
    out_of_set: [u64; 16],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */
impl SmallestInfiniteSet {
    pub fn new() -> Self {
        SmallestInfiniteSet {
            out_of_set: [0; 16],
        }
    }

    pub fn pop_smallest(&mut self) -> i32 {
        let mut index = 0;
        // find 0 bit
        while self.out_of_set[index] == u64::MAX {
            index += 1;
        }
        let temp = self.out_of_set[index];
        let mut offset = 0;
        while temp >> offset & 1 != 0 {
            offset += 1;
        }
        self.out_of_set[index] |= 1 << offset;
        (index * 64 + offset + 1) as i32
    }

    pub fn add_back(&mut self, num: i32) {
        let index = num as usize / 64;
        let offset = num % 64;
        self.out_of_set[index] &= !(1 << offset);
    }
}
