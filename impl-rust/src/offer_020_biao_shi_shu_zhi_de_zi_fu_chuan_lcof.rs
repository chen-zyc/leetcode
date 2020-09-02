struct Solution;

const CHAR_LE: u8 = 'e' as u8;
const CHAR_BE: u8 = 'E' as u8;
const CHAR_0: u8 = '0' as u8;
const CHAR_9: u8 = '9' as u8;
const CHAR_DOT: u8 = '.' as u8;
const CHAR_POS: u8 = '+' as u8;
const CHAR_NEG: u8 = '-' as u8;
const CHAR_SPACE: u8 = ' ' as u8;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum State {
    Initial,
    IntSign,         // + -
    Integer,         // 整数
    Point,           // .
    PointWithoutInt, // . 前没有整数
    Fraction,        // 小数部分
    Exp,             // e E
    ExpSign,         // 指数的符号
    ExpNumber,       // 指数的数字
    End,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum CharType {
    Number,  // 数字
    Exp,     // e E
    Point,   // .
    Sign,    // + -
    Space,   // 空格
    Illegal, // 非法字符
}

impl CharType {
    fn new(c: char) -> Self {
        match c {
            '0'..='9' => CharType::Number,
            'e' | 'E' => CharType::Exp,
            '.' => CharType::Point,
            '+' | '-' => CharType::Sign,
            ' ' => CharType::Space,
            _ => CharType::Illegal,
        }
    }
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        // 状态机
        let mut transfer = HashMap::new();
        transfer.insert(State::Initial, {
            let mut m = HashMap::new();
            m.insert(CharType::Space, State::Initial);
            m.insert(CharType::Number, State::Integer);
            m.insert(CharType::Point, State::PointWithoutInt);
            m.insert(CharType::Sign, State::IntSign);
            m
        });
        transfer.insert(State::IntSign, {
            let mut m = HashMap::new();
            m.insert(CharType::Number, State::Integer);
            m.insert(CharType::Point, State::PointWithoutInt);
            m
        });
        transfer.insert(State::Integer, {
            let mut m = HashMap::new();
            m.insert(CharType::Number, State::Integer);
            m.insert(CharType::Exp, State::Exp);
            m.insert(CharType::Point, State::Point);
            m.insert(CharType::Space, State::End);
            m
        });
        transfer.insert(State::Point, {
            let mut m = HashMap::new();
            m.insert(CharType::Number, State::Fraction);
            m.insert(CharType::Exp, State::Exp);
            m.insert(CharType::Space, State::End);
            m
        });
        transfer.insert(State::PointWithoutInt, {
            let mut m = HashMap::new();
            m.insert(CharType::Number, State::Fraction);
            m
        });
        transfer.insert(State::Fraction, {
            let mut m = HashMap::new();
            m.insert(CharType::Number, State::Fraction);
            m.insert(CharType::Exp, State::Exp);
            m.insert(CharType::Space, State::End);
            m
        });
        transfer.insert(State::Exp, {
            let mut m = HashMap::new();
            m.insert(CharType::Number, State::ExpNumber);
            m.insert(CharType::Sign, State::ExpSign);
            m
        });
        transfer.insert(State::ExpSign, {
            let mut m = HashMap::new();
            m.insert(CharType::Number, State::ExpNumber);
            m
        });
        transfer.insert(State::ExpNumber, {
            let mut m = HashMap::new();
            m.insert(CharType::Number, State::ExpNumber);
            m.insert(CharType::Space, State::End);
            m
        });
        transfer.insert(State::End, {
            let mut m = HashMap::new();
            m.insert(CharType::Space, State::End);
            m
        });

        let mut state = &State::Initial;
        for c in s.chars() {
            let typ = CharType::new(c);
            match transfer.get(state).unwrap().get(&typ) {
                Some(next_state) => state = next_state,
                None => return false,
            }
        }

        match state {
            State::Integer | State::Point | State::Fraction | State::ExpNumber | State::End => true,
            _ => false,
        }
    }

    pub fn is_number1(s: String) -> bool {
        // 直接用库函数。
        if let Ok(_x) = s.trim().parse::<f64>() {
            true
        } else {
            false
        }
    }

    // 情况太多了，硬编码搞不定。
    pub fn is_number_falied(s: String) -> bool {
        // 正则：[+-]?(\d)+ (([eE][+-]?\d+) | ([.]\d+)), 这个正则不完全正确，只是打个样。
        let s = s.as_bytes();
        let mut state = Self::skip_space(s, 0);
        if state.0 == s.len() {
            return false;
        }
        state = Self::have_sign_or_not(s, state.0);
        state = Self::at_least_one_number(s, state.0);
        if state.0 < s.len() {
            match s[state.0] {
                CHAR_LE | CHAR_BE => {
                    // e|E 的前面至少要有一个数字
                    if !state.1 {
                        return false;
                    }
                    state = Self::have_sign_or_not(s, state.0 + 1);
                    state = Self::at_least_one_number(s, state.0);
                }
                CHAR_DOT => {
                    // '.' 前有数字时后面可以没有，后面有时前面可以没有。
                    let prev_have_number = state.1;
                    state = Self::at_least_one_number(s, state.0 + 1);
                    if prev_have_number {
                        state.1 = true;
                    }
                }
                CHAR_SPACE => {}
                _ => return false,
            }
            if !state.1 {
                return false;
            }
        }
        state = Self::skip_space(s, state.0);

        state.1 && state.0 == s.len()
    }

    fn have_sign_or_not(s: &[u8], i: usize) -> (usize, bool) {
        if i >= s.len() {
            return (i, true);
        }
        match s[i] {
            CHAR_POS | CHAR_NEG => (i + 1, true),
            _ => (i, true),
        }
    }

    fn at_least_one_number(s: &[u8], i: usize) -> (usize, bool) {
        let mut j = i;
        while j < s.len() && CHAR_0 <= s[j] && s[j] <= CHAR_9 {
            j += 1;
        }
        (j, j > i)
    }

    fn skip_space(s: &[u8], mut i: usize) -> (usize, bool) {
        while i < s.len() && s[i] == CHAR_SPACE {
            i += 1;
        }
        (i, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_number() {
        assert_eq!(Solution::is_number(" 0".to_owned()), true); // 前后的空格都忽略。
        assert_eq!(Solution::is_number("1 ".to_owned()), true);
        assert_eq!(Solution::is_number(".1".to_owned()), true); // 这也算？
        assert_eq!(Solution::is_number("3.".to_owned()), true); // 这也算？
        assert_eq!(Solution::is_number("+100".to_owned()), true);
        assert_eq!(Solution::is_number("5e2".to_owned()), true);
        assert_eq!(Solution::is_number("-123".to_owned()), true);
        assert_eq!(Solution::is_number("3.1416".to_owned()), true);
        assert_eq!(Solution::is_number("-1E-16".to_owned()), true);
        assert_eq!(Solution::is_number("0123".to_owned()), true);
        assert_eq!(Solution::is_number("46.e3".to_owned()), true);

        assert_eq!(Solution::is_number(" ".to_owned()), false);
        assert_eq!(Solution::is_number("12e".to_owned()), false);
        assert_eq!(Solution::is_number("1a3.14".to_owned()), false);
        assert_eq!(Solution::is_number("1.2.3".to_owned()), false);
        assert_eq!(Solution::is_number("+-5".to_owned()), false);
        assert_eq!(Solution::is_number("12e+5.4".to_owned()), false);
    }
}
