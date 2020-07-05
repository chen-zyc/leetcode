struct Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        // dp[i][j] 表示 s[i] 和 p[j] 是否匹配
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];

        for i in (0..=s.len()).rev() {
            for j in (0..=p.len()).rev() {
                if j >= p.len() {
                    dp[i][j] = i >= s.len();
                } else if i >= s.len() {
                    dp[i][j] = j >= p.len() || (&p[j..]).iter().all(|x| *x == '*' as u8);
                } else if p[j] == '*' as u8 {
                    dp[i][j] = dp[i][j + 1] || dp[i + 1][j];
                } else {
                    let first_match = s[i] == p[j] || p[j] == '?' as u8;
                    dp[i][j] = first_match && dp[i + 1][j + 1];
                }
                println!("i = {}, j = {}, dp = {}", i, j, dp[i][j]);
            }
        }

        dp[0][0]
    }

    // 递归解，会超时。
    pub fn is_match1(s: String, p: String) -> bool {
        Self::is_match_helper(s.as_bytes(), p.as_bytes(), 0, 0)
    }

    fn is_match_helper(s: &[u8], p: &[u8], i: usize, j: usize) -> bool {
        if j >= p.len() {
            return i >= s.len();
        }
        if i >= s.len() {
            // p 也是空的或者 p 都是 *
            return j >= p.len() || (&p[j..]).iter().all(|x| *x == '*' as u8);
        }

        if i < s.len() {
            println!(
                "s = {}, p = {}",
                String::from_utf8_lossy(&s[i..]),
                String::from_utf8_lossy(&p[j..])
            );
        } else {
            println!("p = {}, s is empty", String::from_utf8_lossy(&p[j..]));
        }

        if p[j] == '*' as u8 {
            // 1. * 没有匹配任何字符，p 跳过 * 后再匹配 s[i]
            // 2. 或者 * 匹配上 s[i]，但可能匹配多个，所以用 p[i] 再去匹配 s[i+1]
            (Self::is_match_helper(s, p, i, j + 1)) || (Self::is_match_helper(s, p, i + 1, j))
        } else {
            let first_match = s[i] == p[j] || p[j] == '?' as u8;
            first_match && Self::is_match_helper(s, p, i + 1, j + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match_0040() {
        assert_eq!(Solution::is_match("aa".to_owned(), "a".to_owned()), false);
        assert_eq!(Solution::is_match("aa".to_owned(), "*".to_owned()), true);
        assert_eq!(Solution::is_match("cb".to_owned(), "?a".to_owned()), false);
        assert_eq!(
            Solution::is_match("adceb".to_owned(), "*a*b".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_match("acdcb".to_owned(), "a*c?b".to_owned()),
            false
        );
        assert_eq!(Solution::is_match("ho".to_owned(), "ho**".to_owned()), true);
        assert_eq!(
            Solution::is_match(
                "aaabbbaabaaaaababaabaaabbabbbbbbbbaabababbabbbaaaaba".to_owned(),
                "a*******b".to_owned()
            ),
            false
        );
    }
}
