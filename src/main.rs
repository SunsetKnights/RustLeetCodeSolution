pub mod three_sum_closest;

fn main() {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;
    let result = three_sum_closest::Solution::three_sum_closest(nums, target);
    println!("{}", result);
}
