pub mod pseudo_palindromic_paths;
use pseudo_palindromic_paths::*;

fn main() {
    let result = Solution::pseudo_palindromic_paths(None);
    println!("{:?}", result);
}
