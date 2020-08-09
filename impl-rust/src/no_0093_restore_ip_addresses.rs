struct Solution;
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let n = s.len();
        let mut ans = Vec::new();
        if n < 4 || n > 12 {
            return ans;
        }
        // i 表示在哪个索引位置放置 `.`
        // 每个 for 遍历最多 4 次。注意不要超出 n 的范围。
        for i1 in 1..(n - 2).min(4) {
            if !Self::is_valid(&s[0..i1]) {
                break;
            }
            for i2 in (i1 + 1)..(n - 1).min(i1 + 4) {
                if !Self::is_valid(&s[i1..i2]) {
                    break;
                }
                for i3 in (i2 + 1)..(n.min(i2 + 4)) {
                    if !Self::is_valid(&s[i2..i3]) {
                        break;
                    }
                    if Self::is_valid(&s[i3..n]) {
                        ans.push(Self::to_str(s, i1, i2, i3));
                    }
                }
            }
        }
        ans
    }

    fn is_valid(s: &[u8]) -> bool {
        // 防止下面的计算超出 u32 的范围。
        if s.len() == 0 || s.len() > 3 {
            return false;
        }
        // 只有一个 0 可以，但 00/01 是不行的。
        if s[0] == '0' as u8 {
            return s.len() == 1;
        }
        let n = s
            .iter()
            .fold(0 as u32, |n, c| n * 10 + (*c - '0' as u8) as u32);
        n <= 255
    }

    fn to_str(s: &[u8], i1: usize, i2: usize, i3: usize) -> String {
        let mut ans = String::with_capacity(s.len() + 3);
        ans.extend((&s[0..i1]).iter().map(|u| *u as char));
        ans.push('.');
        ans.extend((&s[i1..i2]).iter().map(|u| *u as char));
        ans.push('.');
        ans.extend((&s[i2..i3]).iter().map(|u| *u as char));
        ans.push('.');
        ans.extend((&s[i3..]).iter().map(|u| *u as char));
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_ip_addresses1() {
        let res = Solution::restore_ip_addresses("25525511135".to_owned());
        let want: Vec<String> = vec!["255.255.11.135", "255.255.111.35"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(res, want);
    }

    #[test]
    fn test_restore_ip_addresses2() {
        let res = Solution::restore_ip_addresses("0000".to_owned());
        let want = vec!["0.0.0.0".to_owned()];
        assert_eq!(res, want);
    }

    #[test]
    fn test_restore_ip_addresses3() {
        let res = Solution::restore_ip_addresses("010010".to_owned());
        let want = vec!["0.10.0.10".to_owned(), "0.100.1.0".to_owned()];
        assert_eq!(res, want);
    }

    #[test]
    fn test_restore_ip_addresses4() {
        let res = Solution::restore_ip_addresses("".to_owned());
        let want: Vec<String> = Vec::new();
        assert_eq!(res, want);
    }
}
