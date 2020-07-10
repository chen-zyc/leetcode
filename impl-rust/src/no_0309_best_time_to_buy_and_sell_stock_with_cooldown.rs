struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut f = (-prices[0], 0, 0);
        for price in prices {
            f = (f.0.max(f.2 - price), f.0 + price, f.1.max(f.2));
        }
        f.1.max(f.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    }
}
