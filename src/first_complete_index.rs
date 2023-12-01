use std::collections::HashMap;

pub struct Solution {}
/**
 * 2661. 找出叠涂元素
 * 给你一个下标从 0 开始的整数数组 arr 和一个 m x n 的整数 矩阵 mat 。arr 和 mat 都包含范围 [1，m * n] 内的 所有 整数。
 * 从下标 0 开始遍历 arr 中的每个下标 i ，并将包含整数 arr[i] 的 mat 单元格涂色。
 * 请你找出 arr 中在 mat 的某一行或某一列上都被涂色且下标最小的元素，并返回其下标 i 。
 */
impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        if arr.len() == 1 {
            return 0;
        }
        let row_count = mat.len();
        let col_count = mat[0].len();
        let mut element_local: HashMap<i32, (usize, usize)> = HashMap::new();
        for (row_i, row) in mat.iter().enumerate() {
            for (col_i, val) in row.iter().enumerate() {
                element_local.insert(*val, (row_i, col_i));
            }
        }
        let mut rows = vec![col_count; row_count];
        let mut cols = vec![row_count; col_count];
        for (i, val) in arr.iter().enumerate() {
            let (row_i, col_i) = element_local[val];
            rows[row_i] -= 1;
            cols[col_i] -= 1;
            if rows[row_i] == 0 || cols[col_i] == 0 {
                return i as i32;
            }
        }
        -1
    }
}
