pub mod min_operations_queries;
use min_operations_queries::*;

fn main() {
    let ret = Solution::min_operations_queries(
        7,
        vec![
            vec![0, 1, 1],
            vec![1, 2, 1],
            vec![2, 3, 1],
            vec![3, 4, 2],
            vec![4, 5, 2],
            vec![5, 6, 2],
        ],
        vec![vec![0, 3], vec![3, 6], vec![2, 6], vec![0, 6]],
    );
    println!("{:#?}", ret);
}
