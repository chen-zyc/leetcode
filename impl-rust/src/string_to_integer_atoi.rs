pub fn my_atoi(str: String) -> i32 {
    // 去除空格
    let mut chars = str.trim_start().chars().peekable();
    let mut res: i32 = 0;
    let mut neg = false;

    if let Some(first) = chars.peek() {
        if *first == '+' || *first == '-' {
            neg = *first == '-';
            chars.next();
        }
    }

    for c in chars {
        match c {
            '0'..='9' => {
                // res = res * 10 + (c - '0')
                if let None = res
                    .checked_mul(10)
                    .and_then(|x| x.checked_add(c.to_digit(10).unwrap() as i32))
                    .map(|x| res = x)
                {
                    return if neg { std::i32::MIN } else { std::i32::MAX };
                }
            }
            _ => break,
        }
    }
    return if neg { -res } else { res };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi() {
        assert_eq!(my_atoi("42".to_owned()), 42);
        assert_eq!(my_atoi("   -42".to_owned()), -42);
        assert_eq!(my_atoi("4193 with words".to_owned()), 4193);
        assert_eq!(my_atoi("words and 987".to_owned()), 0);
        assert_eq!(my_atoi("-91283472332".to_owned()), std::i32::MIN);
        assert_eq!(my_atoi("+-2".to_owned()), 0);
        assert_eq!(my_atoi("2147483648".to_owned()), 2147483647);
    }
}
