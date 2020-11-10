struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() <= 1 {
            return vec![strs];
        }
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for str in strs {
            // 排序
            let mut key = str.clone().into_bytes();
            key.sort();
            let key = unsafe { String::from_utf8_unchecked(key) };
            // 按排序后的 key 分组
            let value = map.entry(key.clone()).or_insert(Vec::new());
            value.push(str);
        }
        let mut ans = Vec::with_capacity(map.len());
        for (_key, value) in map {
            ans.push(value);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::sort_2d_vec;

    #[test]
    fn test_group_anagrams() {
        let strs: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let mut ans = Solution::group_anagrams(strs);
        let mut want = vec![vec!["ate", "eat", "tea"], vec!["nat", "tan"], vec!["bat"]];
        sort_2d_vec(&mut ans);
        sort_2d_vec(&mut want);
        assert_eq!(ans, want);
    }
}
