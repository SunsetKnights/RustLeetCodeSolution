pub mod max_students;
use max_students::*;

fn main() {
    let seats = vec![
        vec!['#', '.', '.', '.', '#'],
        vec!['.', '#', '.', '#', '.'],
        vec!['.', '.', '#', '.', '.'],
        vec!['.', '#', '.', '#', '.'],
        vec!['#', '.', '.', '.', '#'],
    ];
    let ret = Solution::max_students(seats);
    println!("{ret}");
}
