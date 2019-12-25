pub fn reverse(x: i32) -> i32 {
    let mut rev: i32 = 0;
    let mut x = x;
    while x != 0 {
        let pop = x % 10;
        x /= 10;
        // rev = rev * 10 + pop;
        rev = match rev.checked_mul(10).and_then(|x| x.checked_add(pop)) {
            Some(t) => t,
            None => return 0,
        }
    }
    rev
}

#[cfg(test)]
mod test {
    use crate::reverse_integer::reverse;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(120), 21);
        assert_eq!(reverse(std::i32::MAX), 0); // max = 2147483647
    }
}
