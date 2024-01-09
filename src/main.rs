pub mod trie;
use trie::*;

fn main() {
    let mut t = Trie::new();
    t.insert("apple".to_string());
    t.search("apple".to_string());
    t.search("app".to_string());
    t.starts_with("app".to_string());
    t.insert("app".to_string());
    t.search("app".to_string());
}
