mod inner_mutable_tree;
use inner_mutable_tree::*;

fn main() {
    let tree1_val = [3, 5, 1, 6, 2, 9, 8, i32::MAX, i32::MAX, 7, 4];
    let root1 = TreeNode::create_from_vec(tree1_val.to_vec());
    let tree2_val = [
        3,
        5,
        1,
        6,
        7,
        4,
        2,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        9,
        8,
    ];
    let root2 = TreeNode::create_from_vec(tree2_val.to_vec());
    assert_eq!(
        Solution::leaf_similar(root1, root2),
        true,
        "The results are not as expected."
    );
}
