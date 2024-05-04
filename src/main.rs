mod functions_2;
mod functions_3;
mod inner_mutable_tree;

fn main() {
    functions_3::Solution::job_scheduling(
        vec![1, 2, 3, 4, 6],
        vec![3, 5, 10, 6, 9],
        vec![20, 20, 100, 70, 60],
    );
}
