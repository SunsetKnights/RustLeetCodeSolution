pub mod entity_parser;

fn main() {
    let text = String::from("&amp; is an HTML entity but &ambassador; is not.");
    let result = entity_parser::Solution::entity_parser(text);
    println!("{}", result);
}
