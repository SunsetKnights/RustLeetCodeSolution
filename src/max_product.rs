use std::collections::HashMap;

pub struct Solution {}
/**
 * 318. 最大单词长度乘积
 * 给你一个字符串数组 words ，找出并返回 length(words[i]) * length(words[j]) 的最大值，
 * 并且这两个单词不含有公共字母。如果不存在这样的两个单词，返回 0 。
 */
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut temp;
        let mut result = 0;
        for w in words {
            temp = 0;
            for c in w.as_bytes() {
                temp |= 1 << *c - 97;
            }
            let count = map.entry(temp).or_insert(0);
            *count = if *count > w.len() as i32 {
                *count
            } else {
                w.len() as i32
            };
        }
        let keys: Vec<&i32> = map.keys().clone().collect();
        for i in 0..map.len() - 1 {
            for j in i + 1..map.len() {
                if keys[i] & keys[j] == 0 && map[keys[i]] * map[keys[j]] > result {
                    result = map[keys[i]] * map[keys[j]];
                }
            }
        }
        result
    }
}
