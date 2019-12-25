pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return String::new();
    }

    let bytes = s.as_bytes();

    let (mut start, mut end) = (0 as usize, 0 as usize);
    for i in 0..bytes.len() {
        let i = i as isize;
        let len1 = expand_around_center(bytes, i, i);
        let len2 = expand_around_center(bytes, i, i + 1);
        // 下面是把两种情况的处理合在一起了。
        let len = len1.max(len2);
        if len > (end - start) as isize {
            start = (i - (len - 1) / 2) as usize;
            end = (i + len / 2) as usize;
        }
    }

    s[(start as usize)..=(end as usize)].to_owned()
}

fn expand_around_center(s: &[u8], mut left: isize, mut right: isize) -> isize {
    let len = s.len() as isize;
    while left >= 0 && right < len && s[left as usize] == s[right as usize] {
        left -= 1;
        right += 1;
    }
    right - left - 1
}

#[cfg(test)]
mod test {
    use crate::longest_palindrome::longest_palindrome;

    #[test]
    fn test_longest_palindrome() {
        let r = longest_palindrome(String::from("babad"));
        assert!(r == String::from("bab") || r == String::from("aba"));
        assert_eq!(longest_palindrome(String::from("cbbd")), String::from("bb"));
    }
}
