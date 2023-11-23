pub struct Solution {}
/**
 * 1410. HTML 实体解析器
 * 「HTML 实体解析器」 是一种特殊的解析器，它将 HTML 代码作为输入，并用字符本身替换掉所有这些特殊的字符实体。
 * HTML 里这些特殊字符和它们对应的字符实体包括：
 *     双引号：字符实体为 &quot; ，对应的字符是 " 。
 *     单引号：字符实体为 &apos; ，对应的字符是 ' 。
 *     与符号：字符实体为 &amp; ，对应对的字符是 & 。
 *     大于号：字符实体为 &gt; ，对应的字符是 > 。
 *     小于号：字符实体为 &lt; ，对应的字符是 < 。
 *     斜线号：字符实体为 &frasl; ，对应的字符是 / 。
 * 给你输入字符串 text ，请你实现一个 HTML 实体解析器，返回解析器解析后的结果。
 */
impl Solution {
    pub fn entity_parser(text: String) -> String {
        let size = text.len();
        let text = text.as_bytes();
        let mut ret: Vec<u8> = Vec::new();
        let mut index = 0;
        while index < size {
            if text[index] == b'&' {
                if size - index >= 6 && &text[index..index + 6] == "&quot;".as_bytes() {
                    ret.push(b'\"');
                    index += 6;
                    continue;
                } else if size - index >= 6 && &text[index..index + 6] == "&apos;".as_bytes() {
                    ret.push(b'\'');
                    index += 6;
                    continue;
                } else if size - index >= 5 && &text[index..index + 5] == "&amp;".as_bytes() {
                    ret.push(b'&');
                    index += 5;
                    continue;
                } else if size - index >= 4 && &text[index..index + 4] == "&gt;".as_bytes() {
                    ret.push(b'>');
                    index += 4;
                    continue;
                } else if size - index >= 4 && &text[index..index + 4] == "&lt;".as_bytes() {
                    ret.push(b'<');
                    index += 4;
                    continue;
                } else if size - index >= 7 && &text[index..index + 7] == "&frasl;".as_bytes() {
                    ret.push(b'/');
                    index += 7;
                    continue;
                }
            }
            ret.push(text[index]);
            index += 1;
        }
        String::from_utf8(ret).unwrap()
    }
}
