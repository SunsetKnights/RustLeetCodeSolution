pub struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut result = 0;
        for c in s.chars().rev() {
            if c == ' ' {
                if result != 0 {
                    break;
                }
            } else {
                result = result + 1;
            }
        }
        result
    }
}
