struct Solution;
impl Solution {
    // 参考 https://leetcode-cn.com/problems/is-subsequence/solution/rust-0ms-2mb-by-qweytr_1/
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iter_s = s.chars();
        let mut iter_t = t.chars();
        'outer: while let Some(x) = iter_s.next() {
            while let Some(y) = iter_t.next() {
                if x == y {
                    continue 'outer;
                }
            }
            return false;
        }
        true
    }

    // by zhangyuchen. 感觉写的太不 rust 了。
    pub fn is_subsequence1(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut idx_t = 0;
        for i in 0..s.len() {
            let c = s[i];
            // println!("find [{}] {}, idx_t = {}", i, c, idx_t);
            let mut found = false;
            while idx_t < t.len() {
                if c == t[idx_t] {
                    found = true;
                    idx_t += 1;
                    break;
                }
                idx_t += 1;
            }
            if !found {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subsequence1() {
        assert_eq!(
            Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
    }

    #[test]
    fn test_is_subsequence2() {
        assert_eq!(
            Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );
    }

    #[test]
    fn test_is_subsequence3() {
        assert_eq!(
            Solution::is_subsequence("aaaaaa".to_string(), "bbaaaa".to_string()),
            false
        );
    }
}
