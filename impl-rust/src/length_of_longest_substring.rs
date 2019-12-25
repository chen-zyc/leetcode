pub fn length_of_longest_substring(s: String) -> i32 {
    let mut m = std::collections::HashMap::new();
    let mut ans = 0;
    let mut i = 0; // 字串的开始位置。
    for (j, c) in s.chars().enumerate() {
        if let Some(idx) = m.get(&c) {
            // 有重复的字符
            i = i.max(*idx);
        }
        ans = ans.max(j - i + 1); // [i, j] 是想要的子串
        m.insert(c, j + 1); // 如果出现了重复的 c，那么 i 跳到 j+1 的位置。
    }
    ans as i32
}

#[cfg(test)]
mod test {
    use crate::length_of_longest_substring::length_of_longest_substring;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("abba".to_string()), 2);
    }
}
