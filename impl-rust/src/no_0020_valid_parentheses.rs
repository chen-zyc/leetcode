struct Solution;

use std::collections::LinkedList;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = LinkedList::new();
        for c in s.chars() {
            let ok = match c {
                '(' | '[' | '{' => {
                    stack.push_front(c);
                    true
                }
                ')' => stack.pop_front() == Some('('),
                '}' => stack.pop_front() == Some('{'),
                ']' => stack.pop_front() == Some('['),
                _ => false,
            };
            if !ok {
                return false;
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_examples() {
        assert_eq!(Solution::is_valid("()".to_owned()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_owned()), true);
        assert_eq!(Solution::is_valid("(]".to_owned()), false);
        assert_eq!(Solution::is_valid("([)]".to_owned()), false);
        assert_eq!(Solution::is_valid("{[]}".to_owned()), true);
    }
}
