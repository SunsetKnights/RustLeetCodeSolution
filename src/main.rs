mod functions_2;
mod inner_mutable_tree;

fn main() {
    functions_2::Solution::sum_of_distances_in_tree(
        6,
        vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]],
    );
}
