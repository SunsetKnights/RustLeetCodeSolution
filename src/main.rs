pub mod three_sum_closest;

fn main() {
    let nums = vec![1; 10];
    let target = 0;
    let result = three_sum_closest::Solution::three_sum_closest(nums, target);
    println!("{}", result);
}
