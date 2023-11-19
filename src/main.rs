pub mod max_sum_of_three_subarrays;

fn main() {
    let nums = vec![1, 2, 1, 2, 6, 7, 5, 1];
    let k = 2;
    let result = max_sum_of_three_subarrays::Solution::max_sum_of_three_subarrays(nums, k);
    println!("{:?}", result);
}
