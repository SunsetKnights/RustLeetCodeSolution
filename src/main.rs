pub mod maximum_minutes;

fn main() {
    let grid: Vec<Vec<i32>> = vec![
        vec![0, 2, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 2, 2, 1, 0],
        vec![0, 2, 0, 0, 1, 2, 0],
        vec![0, 0, 2, 2, 2, 0, 2],
        vec![0, 0, 0, 0, 0, 0, 0],
    ];
    let result = maximum_minutes::Solution::maximum_minutes(grid);
    println!("{}", result);
}
