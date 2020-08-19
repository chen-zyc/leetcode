struct Solution;
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len() as i32;
        let mut ans = 0;
        for i in 0..(2 * n - 1) {
            // i 是中心点，包括了空隙，所以还要求出对应的索引。
            // [0, 0], [0, 1], [1, 1], [1, 2] ...
            let mut l = i / 2;
            let mut r = l + i % 2;
            // 从中心点向两边扩散。
            while l >= 0 && r < n && s[l as usize] == s[r as usize] {
                ans += 1;
                l -= 1;
                r += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_substrings1() {
        assert_eq!(Solution::count_substrings("abc".to_string()), 3);
    }

    #[test]
    fn test_count_substrings2() {
        assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
    }
}
