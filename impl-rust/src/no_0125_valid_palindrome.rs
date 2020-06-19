struct Solution;

struct BytePos {
    idx: Option<usize>,
    byte: u8,
}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        let chars = s.as_bytes();
        let mut left = 0;
        let mut right = chars.len() - 1;

        while left < right {
            let left_byte = Solution::next_valid_byte(chars, left);
            let right_byte = Solution::prev_valid_byte(chars, right);
            if left_byte.idx.is_none() || right_byte.idx.is_none() {
                break;
            }
            if left_byte.byte != right_byte.byte {
                return false;
            }
            left = left_byte.idx.unwrap() + 1;
            match right_byte.idx {
                Some(0) => break,
                Some(idx) => right = idx - 1,
                None => {}
            };
        }
        true
    }

    fn next_valid_byte(bytes: &[u8], cur_ptr: usize) -> BytePos {
        for i in cur_ptr..bytes.len() {
            match bytes[i] as char {
                'A'..='Z' => {
                    return BytePos {
                        byte: bytes[i] + 32,
                        idx: Some(i),
                    }
                }
                'a'..='z' | '0'..='9' => {
                    return BytePos {
                        byte: bytes[i],
                        idx: Some(i),
                    }
                }
                _ => {}
            }
        }
        return BytePos { byte: 0, idx: None };
    }
    fn prev_valid_byte(bytes: &[u8], cur_ptr: usize) -> BytePos {
        for i in (0..=cur_ptr).rev() {
            match bytes[i] as char {
                'A'..='Z' => {
                    return BytePos {
                        byte: bytes[i] + 32,
                        idx: Some(i),
                    }
                }
                'a'..='z' | '0'..='9' => {
                    return BytePos {
                        byte: bytes[i],
                        idx: Some(i),
                    }
                }
                _ => {}
            }
        }
        return BytePos { byte: 0, idx: None };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_example1() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned()),
            true
        );
    }

    #[test]
    fn test_is_palindrome_example2() {
        assert_eq!(Solution::is_palindrome("race a car".to_owned()), false);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(Solution::is_palindrome("".to_owned()), true);
    }

    #[test]
    fn test_case1() {
        assert_eq!(Solution::is_palindrome("a.".to_owned()), true);
    }
}
