pub mod maximum_sum;

fn main() {
    let nums = vec![18, 43, 36, 13, 7];
    let result = maximum_sum::Solution::maximum_sum(nums);
    println!("{}", result);
}
