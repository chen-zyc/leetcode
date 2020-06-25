struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_dict: HashSet<String> = word_dict.into_iter().collect();
        // dp[i] 表示 s[0..i] 是否符合，不包括 s[i]
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for i in 1..=s.len() {
            let (si, _) = s.split_at(i); // 校验 si 是否符合
            for j in 0..i {
                // j 用来划分 si
                if dp[j] && word_dict.contains(si.split_at(j).1) {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[s.len()]
    }

    // 这种方式在 example4 时超时。
    pub fn word_break1(s: String, word_dict: Vec<String>) -> bool {
        let word_dict: HashSet<String> = word_dict.into_iter().collect();
        Self::check(&s, &word_dict)
    }

    fn check(s: &str, dict: &HashSet<String>) -> bool {
        if s.len() == 0 {
            return true;
        }
        for i in 1..=s.len() {
            let (left, right) = s.split_at(i);
            println!("left = {}, right = {}", left, right);
            if dict.contains(left) && Self::check(right, dict) {
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
    fn test_word_break_example1() {
        assert_eq!(
            Solution::word_break(
                "leetcode".to_owned(),
                vec!["leet", "code"]
                    .into_iter()
                    .map(|s| s.to_owned())
                    .collect(),
            ),
            true
        );
    }

    #[test]
    fn test_word_break_example2() {
        assert_eq!(
            Solution::word_break(
                "applepenapple".to_owned(),
                vec!["apple", "pen"]
                    .into_iter()
                    .map(|s| s.to_owned())
                    .collect(),
            ),
            true
        );
    }

    #[test]
    fn test_word_break_example3() {
        assert_eq!(
            Solution::word_break(
                "catsandog".to_owned(),
                vec!["cats", "dog", "sand", "and", "cat"]
                    .into_iter()
                    .map(|s| s.to_owned())
                    .collect(),
            ),
            false
        );
    }

    #[test]
    fn test_word_break_example4() {
        // 这种容易造成超时
        assert_eq!(
            Solution::word_break(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_owned(),
                vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"]
                    .into_iter()
                    .map(|s| s.to_owned())
                    .collect(),
            ),
            false
        );
    }
}
