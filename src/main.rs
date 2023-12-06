pub mod minimum_total_price;
use minimum_total_price::*;

fn main() {
    let n = 4;
    let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3]];
    let price = vec![2, 2, 10, 6];
    let trips = vec![vec![0, 3], vec![2, 1], vec![2, 3]];
    let result = Solution::minimum_total_price(n, edges, price, trips);
    println!("{}", result);
}
