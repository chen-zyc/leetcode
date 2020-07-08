struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        let mut s = Vec::new();
        Self::backtrack(&mut s, 0, 0, n as usize, &mut ans);
        ans
    }

    fn backtrack(s: &mut Vec<char>, left: usize, right: usize, n: usize, ans: &mut Vec<String>) {
        if s.len() == 2 * n {
            ans.push(s.iter().collect());
            return;
        }
        if left < n {
            s.push('(');
            Self::backtrack(s, left + 1, right, n, ans);
            s.pop();
        }
        if right < left {
            s.push(')');
            Self::backtrack(s, left, right + 1, n, ans);
            s.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        let res = Solution::generate_parenthesis(3);
        let want = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        assert_eq!(res, want);
    }
}
