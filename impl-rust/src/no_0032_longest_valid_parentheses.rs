struct Solution;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut ans = 0;

        let s = s.as_bytes();
        let mut dp = vec![0; s.len()];

        // 第一个肯定凑不成一对括号，所以 dp[0] = 0; 直接从 1 开始判断
        for i in 1..s.len() {
            // 如果是 '('，就让 dp[i] 保持 0 就可以了。只判断 `)`.
            if s[i] == ')' as u8 {
                // 前面就是 '('，那肯定是和当前的 ')' 配成一对的
                if s[i - 1] == '(' as u8 {
                    // s[i-1] -> s[i] 成对了，如果前面紧挨着的也是有效的，那么就能和这俩连在一起了，所以还得加上 dp[i-2]。
                    if i > 2 {
                        dp[i] = 2 + dp[i - 2];
                    } else {
                        dp[i] = 2;
                    }
                } else if i - dp[i - 1] > 0 && s[i - dp[i - 1] - 1] == '(' as u8 {
                    // 前面那个不是 '('，那配对的那个 '(' 可能在更前。
                    // 更前的位置就是把中间成对的都跨过去: i - dp[i-1] - 1。
                    // 这里需要写成 >= 2，因为 i 是 usize，如果把 2 放前面可能会导致溢出。
                    if i - dp[i - 1] >= 2 {
                        dp[i] = dp[i - 1] + 2 + dp[i - dp[i - 1] - 2];
                    } else {
                        dp[i] = dp[i - 1] + 2;
                    }
                }
                ans = ans.max(dp[i]);
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_valid_parentheses1() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_owned()), 2);
    }

    #[test]
    fn test_longest_valid_parentheses2() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_owned()), 4);
    }
}
