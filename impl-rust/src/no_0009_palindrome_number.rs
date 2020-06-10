pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x != 0 && x % 10 == 0) {
        return false;
    }

    let mut n = 0;
    let mut x = x;
    while x > n {
        n = n * 10 + x % 10;
        x /= 10;
    }

    x == n || n / 10 == x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(0), true);
    }
}
