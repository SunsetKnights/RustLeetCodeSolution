pub mod max_taxi_earnings;
use max_taxi_earnings::*;

fn main() {
    let n = 5;
    let rides = vec![vec![2, 5, 4], vec![1, 5, 1]];
    let result = Solution::max_taxi_earnings(n, rides);
    println!("{}", result);
}
