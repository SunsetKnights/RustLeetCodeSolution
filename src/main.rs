mod functions_2;
mod inner_mutable_tree;

fn main() {
    functions_2::Solution::root_count(
        vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]],
        vec![vec![1, 0], vec![3, 4], vec![2, 1], vec![3, 2]],
        1,
    );
}
