pub struct Solution {}
/**
 * 2736. 最大和查询
 * 给你两个长度为 n 、下标从 0 开始的整数数组 nums1 和 nums2 ，另给你一个下标从 1 开始的二维数组 queries ，其中 queries[i] = [xi, yi] 。
 * 对于第 i 个查询，在所有满足 nums1[j] >= xi 且 nums2[j] >= yi 的下标 j (0 <= j < n) 中，找出 nums1[j] + nums2[j] 的 最大值 ，
 * 如果不存在满足条件的 j 则返回 -1 。
 * 返回数组 answer ，其中 answer[i] 是第 i 个查询的答案。
 */
impl Solution {
    pub fn maximum_sum_queries(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        // 把两个数组看成一个平面上点的数组，按照x值降序排列
        let mut a: Vec<(i32, i32)> = nums1.into_iter().zip(nums2.into_iter()).collect();
        a.sort_by(|x, y| y.0.cmp(&x.0));

        // 把待查询的点按照x值降序排列，查询范围即是待查询点的右上部分
        let mut queries_index: Vec<usize> = (0..queries.len()).collect();
        queries_index.sort_by(|&i, &j| queries[j][0].cmp(&queries[i][0]));

        let mut ret = vec![-1; queries.len()];
        let mut sorted_stack: Vec<(i32, i32)> = Vec::new();
        let mut j = 0;
        for &i in &queries_index {
            let x = queries[i][0];
            let y = queries[i][1];
            // 遍历待查询点右边的点
            while j < a.len() && a[j].0 >= x {
                // 若当前点的y值小于栈顶的y，跳过
                // 若当前点的y值大于栈顶的y，后续可能有用，要入栈，入栈前可以把i小于当前点的x+y的点弹出
                // 由于x越来越小，所以如果x+y大于栈顶元素，那么y一定大于栈顶元素，只需要保留y更大的点
                while !sorted_stack.is_empty() && sorted_stack.last().unwrap().1 <= a[j].0 + a[j].1 {
                    sorted_stack.pop();
                }

                if sorted_stack.is_empty() || sorted_stack.last().unwrap().0 < a[j].1 {
                    sorted_stack.push((a[j].1, a[j].0 + a[j].1));
                }
                j += 1;
            }
            let p = sorted_stack.partition_point(|&p| p.0 < y);
            if p < sorted_stack.len() {
                ret[i] = sorted_stack[p].1;
            }
        }
        ret
    }
}
