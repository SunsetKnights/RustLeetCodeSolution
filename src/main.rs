pub mod minimum_effort_path;
use minimum_effort_path::*;

fn main() {
    let heights = vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]];
    let result = Solution::minimum_effort_path(heights);
    println!("{}", result);
}
