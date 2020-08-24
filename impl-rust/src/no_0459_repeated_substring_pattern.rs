struct Solution;
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s = s.as_bytes();
        let n = s.len();
        match n {
            0 | 1 => return false,
            // 子串长度为 3，重复 5 次，15 个字符。
            // 奇数不能重复得到。
            // n if n % 2 == 1 => return false,
            _ => {}
        }

        // 下面这种不行
        // // 比较两边是否相同。
        // let half = s.len() / 2;
        // for i in 0..half {
        //     if s[i] != s[i + half] {
        //         return false;
        //     }
        // }
        // // "ababab" 这样的分成两半是不相等的，但可以由 ab 重复得到。

        for len in (1..=s.len() / 2).rev() {
            // 不能完整的分割。
            if n % len != 0 {
                continue;
            }
            let mut equals = true;
            // 遍历第一个片段的每个字符，它后面的每隔 len 个长度的字符都应该和它相等。
            // 这里没有官方题解中的简洁。
            for i in 0..len {
                let c = s[i];
                let mut j = i + len;
                while j < n {
                    if s[j] != c {
                        equals = false;
                        break;
                    }
                    j += len;
                }
                if !equals {
                    break;
                }
            }
            if equals {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_substring_pattern() {
        assert_eq!(
            Solution::repeated_substring_pattern("abab".to_string()),
            true
        );
        assert_eq!(
            Solution::repeated_substring_pattern("aba".to_string()),
            false
        );
        assert_eq!(
            Solution::repeated_substring_pattern("abcabcabcabc".to_string()),
            true
        );
        assert_eq!(Solution::repeated_substring_pattern("a".to_string()), false);
        assert_eq!(
            Solution::repeated_substring_pattern("ababab".to_string()),
            true
        );
        assert_eq!(
            Solution::repeated_substring_pattern("babbabbabbabbab".to_string()),
            true
        );
        assert_eq!(
            Solution::repeated_substring_pattern("aabaaba".to_string()),
            false
        );
    }
}
