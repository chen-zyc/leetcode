struct Solution;

use std::collections::LinkedList;

impl Solution {
    // 第二遍做了
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            let ok = match c {
                '[' | '{' | '(' => {
                    stack.push(c);
                    true
                }
                ']' => stack.pop() == Some('['),
                '}' => stack.pop() == Some('{'),
                ')' => stack.pop() == Some('('),
                _ => false,
            };
            if !ok {
                return false;
            }
        }
        stack.is_empty()
    }

    pub fn is_valid1(s: String) -> bool {
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
