struct Solution;
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut cnts = HashMap::new();
        for n in arr {
            *cnts.entry(n).or_insert(0) += 1;
        }
        let mut times = HashSet::new();
        for (_, cnt) in cnts.into_iter() {
            // 如果 cnt 不在 times 中，insert 返回 true，否则返回 false
            // 如果 cnt 已经存在于 times 中，则有两个值出现了同样的次数。
            if !times.insert(cnt) {
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
    fn test_unique_occurrences() {
        assert_eq!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
        assert_eq!(Solution::unique_occurrences(vec![1, 2]), false);
        assert_eq!(
            Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
            true
        );
    }
}
