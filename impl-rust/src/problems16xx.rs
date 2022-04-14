struct Solution;
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .into_iter()
            .map(|acc| {
                // 每一行的总和
                acc.iter().sum::<i32>()
            })
            // 每一行的和的最大值
            .max()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_wealth() {
        let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
        assert_eq!(Solution::maximum_wealth(accounts), 6);
    }
}
