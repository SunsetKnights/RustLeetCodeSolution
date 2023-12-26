pub struct Solution;
/**
 * 1349. 参加考试的最大学生数
 * 给你一个 m * n 的矩阵 seats 表示教室中的座位分布。如果座位是坏的（不可用），就用 '#' 表示；否则，用 '.' 表示。
 * 学生可以看到左侧、右侧、左上、右上这四个方向上紧邻他的学生的答卷，
 * 但是看不到直接坐在他前面或者后面的学生的答卷。
 * 请你计算并返回该考场可以容纳的同时参加考试且无法作弊的最大学生人数。
 * 学生必须坐在状况良好的座位上。
 */
impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let row_quantity = seats.len();
        let col_quantity = seats[0].len();
        let row_val_max = ((1usize << col_quantity) - 1) as u8;
        // Store seats as bitmap
        let mut seats_bit = vec![0u8; row_quantity];
        for (i, row) in seats.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == '#' {
                    seats_bit[i] |= 1 << j;
                }
            }
        }
        fn check(curr_row: u8, upper_row: u8, curr_seats: u8) -> bool {
            curr_row & curr_seats == 0
                && curr_row & (curr_row >> 1) == 0
                && (curr_row << 1) & upper_row == 0
                && (curr_row >> 1) & upper_row == 0
                && curr_row & (curr_row << 1) == 0
        }
        fn count(row: u8) -> i32 {
            let mut row = row;
            let mut ret = 0;
            while row != 0 {
                row &= row - 1;
                ret += 1;
            }
            ret
        }
        // Vec<(u8, i32)>
        // u8: Current student seating situation.
        // i32: Maximum number of students.
        let mut upper_rows: Vec<(u8, i32)> = Vec::new();
        for curr_stu in 0..row_val_max {
            if check(curr_stu, 0, seats_bit[0]) {
                upper_rows.push((curr_stu, count(curr_stu)));
            }
        }
        for i in 1..row_quantity {
            let mut curr_rows = Vec::new();
            for curr_situation in 0..=row_val_max {
                let mut curr_max = 0;
                for &(upper_stu, quantity) in &upper_rows {
                    if check(curr_situation, upper_stu, seats_bit[i]) {
                        curr_max = curr_max.max(count(curr_situation) + quantity);
                    }
                }
                curr_rows.push((curr_situation, curr_max));
            }
            upper_rows = curr_rows;
        }
        let mut ret = 0;
        for &(_, quantity) in &upper_rows {
            ret = ret.max(quantity);
        }
        ret
    }
}
