pub mod successful_pairs;

fn main() {
    let spells = vec![3, 1, 2];
    let potions = vec![8, 5, 8];
    let success = 16;
    let result = successful_pairs::Solution::successful_pairs(spells, potions, success);
    println!("{:?}", result);
}
