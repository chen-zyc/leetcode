struct Solution;

const LETTERS: [&[u8]; 8] = [
    b"abc", b"def", b"ghi", b"jkl", b"mno", b"pqrs", b"tuv", b"wxyz",
];
const LETTER_BASE: u8 = '2' as u8;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        if digits.len() > 0 {
            Self::dfs(&digits.as_bytes(), 0, String::new(), &mut ans);
        }
        ans
    }

    fn dfs(digits: &[u8], next_idx: usize, prefix: String, ans: &mut Vec<String>) {
        if next_idx == digits.len() {
            ans.push(prefix);
            return;
        }
        for c in LETTERS[(digits[next_idx] - LETTER_BASE) as usize] {
            let mut new_prefix = prefix.clone();
            new_prefix.push(*c as char);
            Self::dfs(digits, next_idx + 1, new_prefix, ans)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        let mut ans = Solution::letter_combinations(String::from("23"));
        ans.sort();
        let mut want: Vec<String> = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
            .iter()
            .map(|s| String::from(*s))
            .collect();
        want.sort();

        assert_eq!(ans, want);
    }

    #[test]
    fn test_letter_combinations_empty_input() {
        let ans = Solution::letter_combinations(String::new());
        assert_eq!(ans, Vec::<String>::new());
    }
}
