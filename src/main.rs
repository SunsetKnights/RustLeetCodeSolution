pub mod possible_to_stamp;
use possible_to_stamp::*;

fn main() {
    let grid = vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 1]];
    let result = Solution::possible_to_stamp(grid, 3, 3);
    println!("{}", result);
}
