struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut set = HashSet::new();
        while !set.contains(&n) {
            set.insert(n);
            n = Self::square(n);
            if n == 1 {
                return true;
            }
        }

        false
    }

    fn square(mut n: i32) -> i32 {
        let mut ans = 0;
        while n > 0 {
            ans += (n % 10).pow(2);
            n = n / 10;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert_eq!(Solution::is_happy(19), true);
    }
}
