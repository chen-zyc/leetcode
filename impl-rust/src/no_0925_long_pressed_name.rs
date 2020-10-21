struct Solution;
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let mut name_iter = name.chars().peekable();
        let mut prev = None;
        for c1 in typed.chars() {
            if let Some(&c2) = name_iter.peek() {
                if c2 == c1 {
                    name_iter.next();
                    prev = Some(c1);
                    continue;
                }
            }
            if Some(c1) != prev {
                return false;
            }
            prev = Some(c1);
        }
        name_iter.next().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_long_pressed_name() {
        assert_eq!(
            Solution::is_long_pressed_name("alex".to_owned(), "aaleex".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_long_pressed_name("saeed".to_owned(), "ssaaedd".to_owned()),
            false
        );
        assert_eq!(
            Solution::is_long_pressed_name("leelee".to_owned(), "lleeelee".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_long_pressed_name("laiden".to_owned(), "laiden".to_owned()),
            true
        );
        // 如果有剩的，那么剩下的应该是相同的。
        assert_eq!(
            Solution::is_long_pressed_name("alex".to_owned(), "alexxr".to_owned()),
            false
        );
    }
}
