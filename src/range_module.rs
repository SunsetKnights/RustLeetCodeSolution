use std::{collections::BTreeMap, ops::Bound};

pub struct RangeModule {
    ranges: BTreeMap<i32, i32>,
}

/**
 *715. Range 模块
 *Range模块是跟踪数字范围的模块。设计一个数据结构来跟踪表示为 半开区间 的范围并查询它们。
 *半开区间 [left, right) 表示所有 left <= x < right 的实数 x 。
 *实现 RangeModule 类:
 *
 *    RangeModule() 初始化数据结构的对象。
 *    void addRange(int left, int right) 添加 半开区间 [left, right)，跟踪该区间中的每个实数。添加与当前跟踪的数字部分重叠的区间时，应当添加在区间 [left, right) 中尚未跟踪的任何数字到该区间中。
 *    boolean queryRange(int left, int right) 只有在当前正在跟踪区间 [left, right) 中的每一个实数时，才返回 true ，否则返回 false 。
 *    void removeRange(int left, int right) 停止跟踪 半开区间 [left, right) 中当前正在跟踪的每个实数。
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule {
    pub fn new() -> Self {
        Self {
            ranges: BTreeMap::new(),
        }
    }

    pub fn add_range(&mut self, left: i32, right: i32) {
        let right_range = self
            .ranges
            .range((Bound::Unbounded, Bound::Included(&right)));
        let mut right_range: Vec<(i32, i32)> = right_range.map(|(&l, &r)| (l, r)).collect();
        let mut new_left = left;
        let mut new_right = right;
        while let Some((l, r)) = right_range.pop() {
            if r < left {
                break;
            }
            if r > right {
                new_right = r;
            }
            if l < left {
                new_left = l;
            }
            self.ranges.remove(&l);
        }
        self.ranges.insert(new_left, new_right);
    }

    pub fn query_range(&self, left: i32, right: i32) -> bool {
        let mut left_range = self
            .ranges
            .range((Bound::Unbounded, Bound::Excluded(&right)));
        if let Some((&l, &r)) = left_range.next_back() {
            if r >= right && l <= left {
                return true;
            }
        }
        false
    }

    pub fn remove_range(&mut self, left: i32, right: i32) {
        let left_range = self
            .ranges
            .range((Bound::Unbounded, Bound::Excluded(&right)));
        let mut left_range: Vec<(i32, i32)> = left_range.map(|(&l, &r)| (l, r)).collect();
        while let Some((l, r)) = left_range.pop() {
            if r <= left {
                break;
            }
            if l < left {
                let update = self.ranges.get_mut(&l).unwrap();
                *update = left;
            }
            else {
                self.ranges.remove(&l);
            }
            if r > right {
                self.ranges.insert(right, r);
            }
        }
    }
}
