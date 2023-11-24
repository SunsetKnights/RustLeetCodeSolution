pub struct Solution {}
/**
 * 22. 括号生成
 * 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
 */
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut list: Vec<Vec<String>> = vec![Vec::new(); n + 1];
        list[0].push(String::from(""));
        list[1].push(String::from("()"));
        for i in 2..=n {
            for j in 0..i {
                for s_in_i in 0..list[j].len() {
                    let s_in = list[j][s_in_i].clone();
                    for s_right_i in 0..list[i - 1 - j].len() {
                        let s_right = list[i - 1 - j][s_right_i].clone();
                        list[i].push(format!("({}){}", s_in, s_right));
                    }
                }
            }
        }
        list[n].clone()
    }
}
