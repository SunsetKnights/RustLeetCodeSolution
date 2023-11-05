use std::collections::HashMap;

pub struct Solution {}
/**
 * 187. 重复的DNA序列
 * DNA序列 由一系列核苷酸组成，缩写为 'A', 'C', 'G' 和 'T'。
 * 例如，"ACGAATTCCG" 是一个 DNA序列 。
 * 在研究 DNA 时，识别 DNA 中的重复序列非常有用。
 * 给定一个表示 DNA序列 的字符串 s ，返回所有在 DNA 分子中出现不止一次的 长度为 10 的序列(子字符串)。你可以按 任意顺序 返回答案。
 */
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        //把每一个可能的十位序列存入一个hashmap
        //把map中v>=2的加入结果集（这一步跟第一步可以在同一循环中实现）
        //由于只出现4个字母，可以压缩一下存储空间，用0123的二进制码分别代表ACGT，这样每个十位序列只需要一个i32存储
        let s = s.as_bytes();
        let mut counter: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<String> = Vec::new();
        let mut key: i32;
        for i in 10..s.len() + 1 {
            key = 0;
            for c in i - 10..i {
                key |= match s[c] {
                    b'A' => 0,
                    b'C' => 1,
                    b'G' => 2,
                    b'T' => 3,
                    _ => 0,
                } << (i - c - 1) * 2;
            }
            let count = counter.entry(key).or_insert(0);
            *count += 1;
            if *count == 2 {
                result.push(std::str::from_utf8(&s[i - 10..i]).unwrap().to_string());
            }
        }
        result
    }
}
