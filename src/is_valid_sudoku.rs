use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut matrix: HashSet<usize> = HashSet::new();
        let mut g;
        let mut v;
        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    continue;
                }
                g = (r / 3) * 3 + c / 3;
                v = board[r][c].to_string().parse::<usize>().unwrap();
                if matrix.contains(&(r * 9 + v - 1)) {
                    return false;
                }
                matrix.insert(r * 9 + v - 1);
                if matrix.contains(&(81 + c * 9 + v - 1)) {
                    return false;
                }
                matrix.insert(81 + c * 9 + v - 1);
                if matrix.contains(&(162 + g * 9 + v - 1)) {
                    return false;
                }
                matrix.insert(162 + g * 9 + v - 1);
            }
        }
        true
    }
}
