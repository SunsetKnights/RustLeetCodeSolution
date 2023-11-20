pub mod max_sub_array;

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let result = max_sub_array::Solution::max_sub_array(nums);
    println!("{}", result);
}
