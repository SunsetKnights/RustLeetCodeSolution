mod functions_2;
mod functions_3;
mod inner_mutable_tree;

fn main() {
    functions_3::Solution::min_malware_spread_ii(
        vec![
            vec![1, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 1, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 1, 1],
        ],
        vec![3, 7],
    );
}
