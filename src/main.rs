pub mod max_product;

fn main() {
    let nums: Vec<String> = vec![
        "abcw".to_string(),
        "baz".to_string(),
        "foo".to_string(),
        "bar".to_string(),
        "xtfn".to_string(),
        "abcdef".to_string(),
    ];
    let result = max_product::Solution::max_product(nums);
    println!("{}", result);
}
