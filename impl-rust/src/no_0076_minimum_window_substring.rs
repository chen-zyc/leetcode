struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() == 0 || t.len() == 0 {
            return String::new();
        }
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut need = HashMap::new();
        let mut have = HashMap::new();
        for b in t {
            need.entry(b).and_modify(|c| *c += 1).or_insert(1);
        }

        let (mut left, mut right, mut distance) = (0, 0, 0);
        let (mut min, mut start) = (s.len() + 1, 0);
        while right < s.len() {
            let c = s[right];
            println!("c = {}", c as char);
            // 不需要的字符直接略过
            if !need.contains_key(&c) {
                right += 1;
                continue;
            }
            if have.get(&c) < need.get(&c) {
                distance += 1;
                println!("distance = {}", distance);
            }
            have.entry(c).and_modify(|c| *c += 1).or_insert(1);
            // 已经指向一个字符了
            right += 1;
            // 如果这个字符加入到窗口后正好使窗口包含所有字符，那么就需要增加左指针。
            while distance == t.len() {
                // 检查是否是最小窗口
                if right - left < min {
                    min = right - left;
                    start = left;
                    println!("min = {}, start = {}", min, start);
                }
                let c = s[left];
                if !need.contains_key(&c) {
                    left += 1;
                    continue;
                }
                println!("have: {:?}, need: {:?}", have, need);
                if have.get(&c) <= need.get(&c) {
                    distance -= 1;
                }
                have.get_mut(&c).map(|c| *c -= 1);
                left += 1;
            }
        }
        println!("start = {}, min = {}", start, min);
        // 没有找到
        if start + min <= s.len() {
            return String::from_utf8_lossy(&s[start..start + min]).into_owned();
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window1() {
        let ans = Solution::min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned());
        assert_eq!(&ans, "BANC");
    }

    #[test]
    fn test_min_window2() {
        let ans = Solution::min_window("a".to_owned(), "a".to_owned());
        assert_eq!(&ans, "a");
    }

    #[test]
    fn test_min_window3() {
        let ans = Solution::min_window("a".to_owned(), "aa".to_owned());
        assert_eq!(&ans, "");
    }
}
