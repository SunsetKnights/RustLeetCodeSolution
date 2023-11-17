pub mod maximum_sum_queries;

fn main() {
    let nums1 = vec![4, 3, 1, 2];
    let nums2 = vec![2, 4, 9, 5];
    let queries = vec![vec![4, 1], vec![1, 3], vec![2, 5]];
    let result = maximum_sum_queries::Solution::maximum_sum_queries(nums1, nums2, queries);
    println!("{:?}", result);
}
