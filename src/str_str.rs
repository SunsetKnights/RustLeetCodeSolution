pub struct Solution {}
/**
* 28. 找出字符串中第一个匹配项的下标
*给你两个字符串 haystack 和 needle，
*请你在 haystack 字符串中找出 needle 字符串的第一个匹配项的下标（下标从 0 开始）。
*如果 needle 不是 haystack 的一部分，则返回  -1 。
*/
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        //1、部分匹配值表
        let mut vector = vec![0; needle.len()];
        //由于只对英文进行处理，所以直接用字节数组，否则无法随机访问
        let needle_bytes = needle.as_bytes();
        let mut i = 0;
        let mut j = 1;
        let mut same_char = 1;
        while j < needle.len() {
            if needle_bytes[i] == needle_bytes[j] {
                vector[j] = same_char;
                i += 1;
                j += 1;
                same_char += 1;
            } else {
                same_char = 1;
                if i == 0 {
                    j += 1;
                } else {
                    //从不一样的位置往前比较，看是否有相同的子串
                    let mut a = 0;
                    i -= 1;
                    while needle_bytes[i] == needle_bytes[j - a] {
                        if i == 0 {
                            vector[j] = a + 1;
                            j += 1;
                            break;
                        }
                        i -= 1;
                        a += 1;
                    }
                    i = 0;
                }
            }
        }
        let haystack_bytes = haystack.as_bytes();
        i = 0; //起始比较位置
        j = 0; //当前比较位置
        while i + j < haystack.len() {
            if haystack_bytes[i + j] == needle_bytes[j] {
                j += 1;
                if j == needle_bytes.len() {
                    return i as i32;
                }
            } else {
                if j == 0 || vector[j - 1] == 0 {
                    i += 1;
                    j = 0;
                } else {
                    i = i + j - vector[j - 1];
                    j = vector[j - 1];
                }
            }
        }
        -1
    }
}
