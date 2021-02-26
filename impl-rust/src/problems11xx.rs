struct Solution;
impl Solution {
    // 1178. 猜字谜: https://leetcode-cn.com/problems/number-of-valid-words-for-each-puzzle/
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        // 参考官方题解。
        const PUZZLE_LEN: u32 = 7;

        // 计算 words 的二进制。
        let mut cnt = std::collections::HashMap::new();
        for s in words {
            let mask = s.bytes().fold(0_u32, |s, c| s | (1 << (c - 'a' as u8)));
            if mask.count_ones() <= PUZZLE_LEN {
                cnt.entry(mask).and_modify(|v| *v += 1).or_insert(1);
            }
        }

        let mut ans = vec![0; puzzles.len()];
        for (i, puzzle) in puzzles.into_iter().enumerate() {
            let puzzle = puzzle.as_bytes();
            let first: u32 = 1 << (puzzle[0] - 'a' as u8);

            let mut mask = 0_u32;
            for b in &puzzle[1..] {
                mask |= 1 << (b - 'a' as u8);
            }

            let mut subset = mask;
            loop {
                ans[i] += cnt.get(&(subset | first)).unwrap_or(&0);
                subset = (subset.wrapping_sub(1)) & mask;
                if subset == mask {
                    break;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_num_of_valid_words() {
        let words = vec!["aaaa", "asas", "able", "ability", "actt", "actor", "access"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let puzzles = vec![
            "aboveyz", "abrodyz", "abslute", "absoryz", "actresz", "gaswxyz",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        let ans = Solution::find_num_of_valid_words(words, puzzles);
        let want = vec![1, 1, 3, 2, 4, 0];
        assert_eq!(ans, want);
    }
}
