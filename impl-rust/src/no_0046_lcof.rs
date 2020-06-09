pub fn translate_num(num: i32) -> i32 {
    if num < 10 {
        return 1;
    }

    let s = num.to_string();
    let mut prev = (0, 1, 1);
    for i in 1..s.len() {
        prev.0 = prev.1;
        prev.1 = prev.2;
        let prev_str = &s[i - 1..=i];
        if "10" <= prev_str && prev_str <= "25" {
            prev.2 = prev.1 + prev.0;
        } else {
            prev.2 = prev.1;
        }
    }
    prev.2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_translate_num() {
        assert_eq!(translate_num(1), 1);
        assert_eq!(translate_num(10), 2);
        assert_eq!(translate_num(25), 2);
        assert_eq!(translate_num(12258), 5);
    }
}
