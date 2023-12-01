pub mod first_complete_index;
use first_complete_index::*;

fn main() {
    let arr = vec![1, 3, 4, 2];
    let mat = vec![vec![1, 4], vec![2, 3]];
    let result = Solution::first_complete_index(arr, mat);
    println!("{:?}", result);
}
