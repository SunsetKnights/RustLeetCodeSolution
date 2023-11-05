pub struct Solution {}
impl Solution {
    /**
     *给你一个整数数组 citations ，其中 citations[i] 表示研究者的第 i 篇论文被引用的次数。计算并返回该研究者的 h 指数。
     *根据维基百科上 h 指数的定义：h 代表“高引用次数” ，一名科研人员的 h 指数 是指他（她）至少发表了 h 篇论文，
     *并且每篇论文 至少 被引用 h 次。如果 h 有多种可能的值，h 指数 是其中最大的那个。
     */
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut sortable = citations.clone();
        sortable.sort_unstable();
        let size: i32 = sortable.len() as i32;
        let mut low: i32 = 0;
        let mut high: i32 = size - 1;
        let mut mid: i32 = (low + high) / 2;
        while low <= high {
            if sortable[mid as usize] < (size - mid) {
                low = mid + 1;
            } else if sortable[mid as usize] > (size - mid) {
                high = mid - 1;
            } else {
                return size - mid;
            }
            mid = (low + high) / 2;
        }
        size - low
    }
}
