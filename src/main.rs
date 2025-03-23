mod functions_4;
mod inner_mutable_tree;
mod lru_cache;

fn main() {
    inner_mutable_tree::Solution::max_path_sum(inner_mutable_tree::TreeNode::from_vec(vec![
        Some(1),
        Some(2),
        Some(2),
    ]));
}
