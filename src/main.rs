pub mod find_the_city;

fn main() {
    let n = 4;
    let edges = vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]];
    let distance_threshold = 4;
    let result = find_the_city::Solution::find_the_city(n, edges, distance_threshold);
    println!("{}", result);
}
