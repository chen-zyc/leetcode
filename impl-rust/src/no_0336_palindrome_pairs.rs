struct Solution;
impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let words: Vec<&[u8]> = words.iter().map(|word| word.as_bytes()).collect();
        let n = words.len();
        let mut res = Vec::new();

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue; // 要求是不同的索引对
                }
                if Self::is_palindrome(words[i], words[j]) {
                    res.push(vec![i as i32, j as i32]);
                }
            }
        }

        res
    }

    fn is_palindrome(s1: &[u8], s2: &[u8]) -> bool {
        let (m, n) = (s1.len(), s2.len());
        if m == n && m == 0 {
            return true; // 两个空字符串是回文
        }
        let (mut l, mut r) = (0, m + n - 1);
        while l <= r {
            let c1 = if l < m { s1[l] } else { s2[l - m] };
            let c2 = if r < m { s1[r] } else { s2[r - m] };
            if c1 != c2 {
                return false;
            }
            l += 1;
            if r == 0 {
                break;
            }
            r -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_pairs1() {
        let words: Vec<String> = vec!["abcd", "dcba", "lls", "s", "sssll"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let mut ans = Solution::palindrome_pairs(words);
        ans.sort_by(|w1, w2| w1[0].cmp(&w2[0])); // 顺序无关
        let mut want = vec![vec![0, 1], vec![1, 0], vec![3, 2], vec![2, 4]];
        want.sort_by(|w1, w2| w1[0].cmp(&w2[0]));
        assert_eq!(ans, want);
    }

    #[test]
    fn test_palindrome_pairs2() {
        let words: Vec<String> = vec!["bat", "tab", "cat"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let mut ans = Solution::palindrome_pairs(words);
        ans.sort_by(|w1, w2| w1[0].cmp(&w2[0]));
        let mut want = vec![vec![0, 1], vec![1, 0]];
        want.sort_by(|w1, w2| w1[0].cmp(&w2[0]));
        assert_eq!(ans, want);
    }

    #[test]
    fn test_palindrome_pairs3() {
        let words: Vec<String> = vec!["a", ""].into_iter().map(|s| s.to_string()).collect();
        let mut ans = Solution::palindrome_pairs(words);
        ans.sort_by(|w1, w2| w1[0].cmp(&w2[0]));
        let mut want = vec![vec![0, 1], vec![1, 0]];
        want.sort_by(|w1, w2| w1[0].cmp(&w2[0]));
        assert_eq!(ans, want);
    }
}
