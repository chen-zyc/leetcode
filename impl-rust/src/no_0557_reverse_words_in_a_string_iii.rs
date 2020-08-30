struct Solution;
impl Solution {
    // 直接在原字符串上修改
    pub fn reverse_words(mut s: String) -> String {
        let bytes = unsafe { s.as_mut_vec() };
        let n = bytes.len();
        let mut i = 0;
        while i < n {
            // 单词的开始
            let mut start = i;
            // 让 i 指向第一个空格的位置
            while i < n && bytes[i] != ' ' as u8 {
                i += 1;
            }
            // [start, end] 就是单词的范围
            let mut end = i - 1;
            while start < end {
                // 交换 start 和 end 的字母
                bytes.swap(start, end);
                start += 1;
                end -= 1;
            }
            // 跳过 i 及之后的空格
            while i < n && bytes[i] == ' ' as u8 {
                i += 1;
            }
        }
        s
    }

    // 另开辟一个空间存储答案
    pub fn reverse_words1(s: String) -> String {
        let s = s.as_bytes();
        let n = s.len();
        let (mut i, mut j) = (0, 0);
        let mut ans = String::new();
        while j < n {
            // j 指向第一个空格
            while j < n && s[j] != ' ' as u8 {
                j += 1;
            }
            // 把 j 之前的字符倒序插入答案中
            for k in (i..j).rev() {
                ans.push(s[k] as char);
            }
            // 把 j 之后的空格插入答案中
            while j < n && s[j] == ' ' as u8 {
                ans.push(s[j] as char);
                j += 1;
            }
            i = j;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_owned()),
            "s'teL ekat edoCteeL tsetnoc".to_owned()
        );
    }
}
