struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        match strs.len() {
            0 => return String::new(),
            1 => return strs[0].clone(),
            _ => (),
        };

        let mut buf = String::new();
        for (i, c) in strs[0].as_bytes().iter().enumerate() {
            for j in 1..strs.len() {
                if i >= strs[j].len() || strs[j].as_bytes()[i] != *c {
                    return buf;
                }
            }
            buf.push(*c as char);
        }

        buf
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let strs = vec!["flower", "flow", "flight"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::longest_common_prefix(strs), "fl".to_owned());

        let strs = vec!["dog", "racecar", "car"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::longest_common_prefix(strs), "".to_owned());

        let strs = vec!["aa", "a"].iter().map(|x| x.to_string()).collect();
        assert_eq!(Solution::longest_common_prefix(strs), "a".to_owned());
    }
}
