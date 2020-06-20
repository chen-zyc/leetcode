struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::is_match_bytes(s.as_bytes(), p.as_bytes())
    }

    fn is_match_bytes(s: &[u8], p: &[u8]) -> bool {
        if p.len() == 0 {
            return s.len() == 0;
        }

        let first_match = s.len() != 0 && (s[0] == p[0] || p[0] == '.' as u8);

        if p.len() > 1 && p[1] == '*' as u8 {
            return (first_match && Self::is_match_bytes(&s[1..], p))
                || (Self::is_match_bytes(s, &p[2..]));
        }

        first_match && Self::is_match_bytes(&s[1..], &p[1..])
    }

    pub fn is_match2(s: String, p: String) -> bool {
        let mut memo = vec![vec![None; p.len() + 1]; s.len() + 1];
        Self::dp2(s.as_bytes(), p.as_bytes(), 0, 0, &mut memo)
    }

    fn dp2(s: &[u8], p: &[u8], i: usize, j: usize, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
        if memo[i][j].is_some() {
            return memo[i][j].unwrap();
        }

        let ans;
        if j == p.len() {
            ans = i == s.len();
        } else {
            let first_match = i < s.len() && (s[i] == p[j] || p[j] == '.' as u8);
            if j + 1 < p.len() && p[j + 1] == '*' as u8 {
                ans = (first_match && Self::dp2(s, p, i + 1, j, memo))
                    || (Self::dp2(s, p, i, j + 2, memo))
            } else {
                ans = first_match && Self::dp2(s, p, i + 1, j + 1, memo)
            }
        }

        memo[i][j] = Some(ans);
        ans
    }

    pub fn is_match3(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let (slen, plen) = (s.len(), p.len());
        let mut memo = vec![vec![None; plen + 1]; slen + 1]; // 这里不需要用 Option, 见 is_match4
        memo[slen][plen] = Some(true);

        for i in (0..=slen).rev() {
            for j in (0..plen).rev() {
                let first_match = i < slen && (s[i] == p[j] || p[j] == '.' as u8);
                if j + 1 < plen && p[j + 1] == '*' as u8 {
                    memo[i][j] = Some(
                        (first_match && memo[i + 1][j] == Some(true))
                            || memo[i][j + 2] == Some(true),
                    );
                } else {
                    memo[i][j] = Some(first_match && memo[i + 1][j + 1] == Some(true));
                }
                // println!("m[{}][{}] = {:?}", i, j, memo[i][j]);
            }
        }

        memo[0][0] == Some(true)
    }

    // 2020-06-20 再做一次，独立完成。
    pub fn is_match4(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();

        let mut memo = vec![vec![false; p.len() + 1]; s.len() + 1];
        memo[s.len()][p.len()] = true;

        for i in (0..=s.len()).rev() {
            for j in (0..p.len()).rev() {
                // 现在要计算 memo[i][j]
                let first_match = i < s.len() && (s[i] == p[j] || (p[j] == '.' as u8));
                if j + 1 < p.len() && p[j + 1] == '*' as u8 {
                    memo[i][j] = memo[i][j + 2] || (first_match && memo[i + 1][j]);
                } else {
                    memo[i][j] = first_match && memo[i + 1][j + 1];
                }
                // println!("m[{}][{}] = {}", i, j, memo[i][j]);
            }
        }
        memo[0][0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_match() {
        assert_eq!(Solution::is_match("aa".to_owned(), "a*".to_owned()), true);
        assert_eq!(
            Solution::is_match("aab".to_owned(), "c*a*b".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_match("mississippi".to_owned(), "mis*is*p*".to_owned()),
            false
        );
    }

    #[test]
    fn test_is_match2() {
        assert_eq!(Solution::is_match2("aa".to_owned(), "a*".to_owned()), true);
        assert_eq!(
            Solution::is_match2("aab".to_owned(), "c*a*b".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_match2("mississippi".to_owned(), "mis*is*p*".to_owned()),
            false
        );
        assert_eq!(
            Solution::is_match2("ab".to_owned(), ".*c".to_owned()),
            false
        );
    }

    #[test]
    fn test_is_match3() {
        assert_eq!(Solution::is_match3("aa".to_owned(), "a*".to_owned()), true);
        assert_eq!(
            Solution::is_match3("aab".to_owned(), "c*a*b".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_match3("mississippi".to_owned(), "mis*is*p*".to_owned()),
            false
        );
        assert_eq!(
            Solution::is_match3("ab".to_owned(), ".*c".to_owned()),
            false
        );
    }

    #[test]
    fn test_is_match4() {
        assert_eq!(Solution::is_match4("ab".to_owned(), ".*".to_owned()), true);
        // assert_eq!(Solution::is_match3("ab".to_owned(), ".*".to_owned()), true);
    }
}
