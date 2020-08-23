struct Solution;
impl Solution {
    // 消除最右边的 1
    pub fn range_bitwise_and(m: i32, mut n: i32) -> i32 {
        while m < n {
            // 使用 &= 性能提升很多？？
            // n = n & (n - 1);
            n &= n - 1;
        }
        n
    }

    // 位移法。
    pub fn range_bitwise_and2(mut m: i32, mut n: i32) -> i32 {
        let mut count = 0;
        while m != n {
            m >>= 1;
            n >>= 1;
            count += 1;
        }
        n << count
    }

    // 暴力法。
    pub fn range_bitwise_and1(m: i32, n: i32) -> i32 {
        (m..=n).fold(std::i32::MAX, |acc, n| acc & n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_bitwise_and1() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
        assert_eq!(Solution::range_bitwise_and(0, 1), 0);
    }
}
