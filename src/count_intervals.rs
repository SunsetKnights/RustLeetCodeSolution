use std::{collections::BTreeMap, ops::Bound};

pub struct CountIntervals {
    // key: left
    // value: right
    ranges: BTreeMap<i32, i32>,
    len: i32,
}

/**
 * 2276. 统计区间中的整数数目
 * 给你区间的 空 集，请你设计并实现满足要求的数据结构：
 *
 *     新增：添加一个区间到这个区间集合中。
 *     统计：计算出现在 至少一个 区间中的整数个数。
 *
 * 实现 CountIntervals 类：
 *
 *     CountIntervals() 使用区间的空集初始化对象
 *     void add(int left, int right) 添加区间 [left, right] 到区间集合之中。
 *     int count() 返回出现在 至少一个 区间中的整数个数。
 *
 * 注意：区间 [left, right] 表示满足 left <= x <= right 的所有整数 x 。
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CountIntervals {
    pub fn new() -> Self {
        Self {
            ranges: BTreeMap::new(),
            len: 0,
        }
    }

    fn add(&mut self, left: i32, right: i32) {
        // last right_range.left <= right
        let mut right_range = self
            .ranges
            .range((Bound::Unbounded, Bound::Included(&right)));
        let mut new_left = left;
        let mut new_right = right;
        let mut removed:Vec<i32> = Vec::new();
        while let Some((&l, &r)) = right_range.next_back() {
            if r < left {
                break;
            }
            self.len -= r - l + 1;
            if r > right {
                new_right = r;
            }
            if l < left {
                new_left = l;
            }
            removed.push(l);
        }
        drop(right_range);
        while let Some(k) = removed.pop() {
            self.ranges.remove(&k);
        }
        self.ranges.insert(new_left, new_right);
        self.len += new_right - new_left + 1;
    }

    fn count(&self) -> i32 {
        self.len
    }
}
