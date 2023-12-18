pub struct Solution;
/**
 * 37. 解数独
 * 编写一个程序，通过填充空格来解决数独问题。
 * 数独的解法需 遵循如下规则：
 *     数字 1-9 在每一行只能出现一次。
 *     数字 1-9 在每一列只能出现一次。
 *     数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。
 * 数独部分空格内已填入了数字，空白格用 '.' 表示。
 */
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut converted_soduku = Vec::with_capacity(81);
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    converted_soduku.push(0);
                } else {
                    converted_soduku.push(board[i][j] as u8 - '0' as u8);
                }
            }
        }
        fn check(soduku: &Vec<u8>, put_num: (usize, usize, u8)) -> bool {
            let row = put_num.0;
            let col = put_num.1;
            let val = put_num.2;
            // check row
            for i in row * 9..row * 9 + 9 {
                if soduku[i] == val {
                    return false;
                }
            }
            // check column
            for i in (col..80).step_by(9) {
                if soduku[i] == val {
                    return false;
                }
            }
            // check area
            for i in row / 3 * 3..row / 3 * 3 + 3 {
                for j in col / 3 * 3..col / 3 * 3 + 3 {
                    if soduku[i * 9 + j] == val {
                        return false;
                    }
                }
            }
            true
        }
        fn solve(curr_soduku: &mut Vec<u8>) -> bool {
            // find empty cell
            let mut empty_cell = 0;
            while empty_cell < 81 && curr_soduku[empty_cell] != 0 {
                empty_cell += 1;
            }
            if empty_cell == 81 {
                return true;
            }
            for val in 1..=9 {
                if check(curr_soduku, (empty_cell / 9, empty_cell % 9, val)) {
                    curr_soduku[empty_cell] = val;
                    if solve(curr_soduku) {
                        return true;
                    }
                    curr_soduku[empty_cell] = 0;
                }
            }
            return false;
        }

        solve(&mut converted_soduku);

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    board[i][j] = char::from(converted_soduku[i * 9 + j] + '0' as u8);
                }
            }
        }
    }
}
