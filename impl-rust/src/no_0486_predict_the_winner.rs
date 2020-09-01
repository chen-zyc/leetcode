struct Solution;
impl Solution {
    // 使用滚动数组优化动态规划。
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let m = nums.len();
        let mut dp = vec![0; m];
        for i in (0..m - 1).rev() {
            // 每计算一行时，先算 i == j 时的值。
            dp[i] = nums[i];
            for j in i + 1..m {
                // dp[i+1][j] 是下一行的当前位置，因为还没重新赋值，所以可以直接用 dp[j]。
                // dp[i][j-1] 就是前一个。
                dp[j] = (nums[i] - dp[j]).max(nums[j] - dp[j - 1]);
            }
        }
        dp[m - 1] >= 0
    }

    // 动态规划解法
    pub fn predict_the_winner2(nums: Vec<i32>) -> bool {
        // 题目保证 m > 0
        let m = nums.len();
        // dp[i][j] 是 nums[i..=j] 时玩家能获得的最大分数。(不一定是玩家1)
        let mut dp = vec![vec![0; m]; m];
        // 当 i == j 时，玩家只有一个选择，所以分数就是 nums[i].
        for i in 0..m {
            dp[i][i] = nums[i];
        }
        // 当 i < j 时，玩家可以选择 [i]，也可以选择 [j].
        for i in (0..m - 1).rev() {
            for j in i + 1..m {
                dp[i][j] = (nums[i] - dp[i + 1][j]).max(nums[j] - dp[i][j - 1]);
            }
        }
        dp[0][m - 1] >= 0
    }

    // 递归解法。
    pub fn predict_the_winner1(nums: Vec<i32>) -> bool {
        Self::total(&nums, 0, nums.len() - 1, 1) >= 0
    }

    fn total(nums: &Vec<i32>, start: usize, end: usize, turn: i32) -> i32 {
        if start == end {
            return nums[start] * turn;
        }
        // 选择 start
        let score1 = nums[start] * turn + Self::total(nums, start + 1, end, -turn);
        // 选择 end
        let score2 = nums[end] * turn + Self::total(nums, start, end - 1, -turn);
        // 如果是玩家1，那么选择两者最大的那个。
        // 如果是玩家2，score 是负数，应该选择最小的那个，对玩家2来说才是最优选择。
        if turn == 1 {
            score1.max(score2)
        } else {
            score1.min(score2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_the_winner1() {
        assert_eq!(Solution::predict_the_winner(vec![1, 5, 2]), false);
        assert_eq!(Solution::predict_the_winner(vec![1, 5, 233, 7]), true);
    }
}
