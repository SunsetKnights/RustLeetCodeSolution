pub struct Solution;
/**
 * 2008. 出租车的最大盈利
 * 你驾驶出租车行驶在一条有 n 个地点的路上。这 n 个地点从近到远编号为 1 到 n ，你想要从 1 开到 n ，通过接乘客订单盈利。你只能沿着编号递增的方向前进，不能改变方向。
 * 乘客信息用一个下标从 0 开始的二维数组 rides 表示，其中 rides[i] = [starti, endi, tipi] 表示第 i 位乘客需要从地点 starti 前往 endi ，愿意支付 tipi 元的小费。
 * 每一位 你选择接单的乘客 i ，你可以 盈利 endi - starti + tipi 元。你同时 最多 只能接一个订单。
 * 给你 n 和 rides ，请你返回在最优接单方案下，你能盈利 最多 多少元。
 * 注意：你可以在一个地点放下一位乘客，并在同一个地点接上另一位乘客。
 */
impl Solution {
    pub fn max_taxi_earnings(_n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let mut max = vec![0i64; rides.len() + 1];
        let mut rides: Vec<(i32, i32, i32)> = rides.iter().map(|x| (x[1], x[0], x[2])).collect();
        rides.sort_unstable();
        for i in 1..=rides.len() {
            let curr = &rides[i - 1];
            let front = rides.partition_point(|&p| p.0 <= curr.1);
            let include_curr = (curr.0 - curr.1 + curr.2) as i64 + max[front];
            let not_include_curr = max[i - 1];
            max[i] = include_curr.max(not_include_curr);
        }
        max[rides.len()]
    }
}
