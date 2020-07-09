struct Solution;
impl Solution {
    pub fn respace(dictionary: Vec<String>, sentence: String) -> i32 {
        let mut root = Trie::new();
        for word in dictionary {
            root.insert(&word);
        }

        let sentence = sentence.as_bytes();
        let n = sentence.len();
        let mut dp = vec![std::i32::MAX; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            dp[i] = dp[i - 1] + 1;

            let mut pos = &mut root;
            for j in (1..=i).rev() {
                let t = sentence[j - 1] as usize - 'a' as usize;
                if pos.next[t].is_none() {
                    break;
                } else if pos.next[t].as_ref().unwrap().is_end {
                    dp[i] = dp[i].min(dp[j - 1]);
                }
                if dp[i] == 0 {
                    break;
                }
                pos = pos.next[t].as_mut().unwrap();
            }
        }

        dp[n]
    }
}

struct Trie {
    next: Vec<Option<Box<Trie>>>,
    is_end: bool,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            next: (0..26).map(|_| None).collect(),
            is_end: false,
        }
    }

    fn insert(&mut self, s: &str) {
        // 倒序插入树中。
        let mut pos = self;
        for c in s.chars().rev() {
            let t = c as usize - 'a' as usize;
            pos = pos.next[t].get_or_insert(Box::new(Trie::new()));
        }
        pos.is_end = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_respace1() {
        assert_eq!(
            Solution::respace(
                vec!["looked", "just", "like", "her", "brother"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                "jesslookedjustliketimherbrother".to_string(),
            ),
            7
        );
    }
}
