struct Solution;
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut ans = 0;
        let mut idx = 0;
        let mut prev = 0;
        while idx < n {
            let c = s[idx];
            let mut count = 0;
            while idx < n && s[idx] == c {
                count += 1;
                idx += 1;
            }
            ans += count.min(prev);
            prev = count;
        }
        ans
    }

    // 超时了。
    pub fn count_binary_substrings_failed(s: String) -> i32 {
        let mut ans = 0;
        let s = s.as_bytes();
        let n = s.len();

        for start in 0..(n - 1) {
            let num = s[start];
            let mut end = start + 1; // 另一个数字的开始索引
            while end < n && s[end] == num {
                end += 1;
            }
            // 计算 num 的个数
            let count = end - start;

            // 在后面查找另一个数字的个数
            let mut count2 = 0;
            while end < n && s[end] != num && count2 < count {
                count2 += 1;
                end += 1;
            }

            if count2 == count {
                ans += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_binary_substrings1() {
        assert_eq!(Solution::count_binary_substrings("00110011".to_owned()), 6);
    }

    #[test]
    fn test_count_binary_substrings2() {
        assert_eq!(Solution::count_binary_substrings("10101".to_owned()), 4);
    }

    #[test]
    fn test_count_binary_substrings3() {
        assert_eq!(Solution::count_binary_substrings("00110".to_owned()), 3);
    }
}
