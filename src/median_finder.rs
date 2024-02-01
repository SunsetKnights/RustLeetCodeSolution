use std::{cmp::Reverse, collections::BinaryHeap};

/**
 * 295. 数据流的中位数
 * 中位数是有序整数列表中的中间值。如果列表的大小是偶数，则没有中间值，中位数是两个中间值的平均值。
 *     例如 arr = [2,3,4] 的中位数是 3 。
 *     例如 arr = [2,3] 的中位数是 (2 + 3) / 2 = 2.5 。
 *
 * 实现 MedianFinder 类:
 *     MedianFinder() 初始化 MedianFinder 对象。
 *     void addNum(int num) 将数据流中的整数 num 添加到数据结构中。
 *     double findMedian() 返回到目前为止所有元素的中位数。与实际答案相差 10-5 以内的答案将被接受。
 */

pub struct MedianFinder {
    smaller_half: BinaryHeap<i32>,
    bigger_half: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    pub fn new() -> Self {
        Self {
            smaller_half: BinaryHeap::new(),
            bigger_half: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        match self.smaller_half.len() == self.bigger_half.len() {
            true => {
                self.smaller_half.push(num);
                let mid = self.smaller_half.pop().unwrap();
                self.bigger_half.push(Reverse(mid));
            }
            false => {
                self.bigger_half.push(Reverse(num));
                let mid = self.bigger_half.pop().unwrap();
                self.smaller_half.push(mid.0);
            }
        };
    }

    pub fn find_median(&self) -> f64 {
        match self.smaller_half.len() == self.bigger_half.len() {
            true => {
                (self.bigger_half.peek().unwrap().0 as f64
                    + *self.smaller_half.peek().unwrap() as f64)
                    / 2.0
            }
            false => self.bigger_half.peek().unwrap().0 as f64,
        }
    }
}
