pub struct Solution {}
/**
 *
 */
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits_changeable = digits.clone();
        let mut i = digits.len();
        while i > 0 && digits_changeable[i - 1] + 1 == 10 {
            digits_changeable[i - 1] = 0;
            i -= 1;
        }
        if i == 0 {
            digits_changeable.insert(0, 1);
        } else {
            digits_changeable[i - 1] += 1;
        }

        digits_changeable
    }
}
