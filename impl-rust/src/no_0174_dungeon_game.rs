struct Solution;
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        if dungeon.is_empty() {
            return 0;
        }
        let (m, n) = (dungeon.len(), dungeon[0].len());
        // 边界除了 [m, n-1] 和 [m-1, n] 外都是最大值
        let mut dp = vec![vec![std::i32::MAX; n + 1]; m + 1];
        dp[m][n - 1] = 1;
        dp[m - 1][n] = 1;

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let minn = dp[i][j + 1].min(dp[i + 1][j]);
                dp[i][j] = (minn - dungeon[i][j]).max(1);
            }
        }

        dp[0][0]
    }

    // by zhangyuchen, 从左上到右下的方式解不出来 😭️
    pub fn calculate_minimum_hp_failed(dungeon: Vec<Vec<i32>>) -> i32 {
        if dungeon.is_empty() {
            return 0;
        }
        let (m, n) = (dungeon.len(), dungeon[0].len());
        // .0 是指如果血量开始为0，那么到当前点时血量是多少
        // .1 是指到当前点时最少需要的血量
        let mut res = vec![(0, 0); n];

        for i in 0..m {
            for j in 0..n {
                res[j] = if i > 0 && j > 0 {
                    // 这里的策略应该怎么选呢？选择血量最多的？还是选择点数最少的？
                    let top = Self::gen_health_point(res[j], dungeon[i][j]);
                    let left = Self::gen_health_point(res[j - 1], dungeon[i][j]);
                    if top.0 > left.0 {
                        top
                    } else {
                        left
                    }
                } else if i > 0 {
                    // 只能从上边到这里
                    Self::gen_health_point(res[j], dungeon[i][j])
                } else if j > 0 {
                    // 只能从左边到这里
                    Self::gen_health_point(res[j - 1], dungeon[i][j])
                } else {
                    // 只有自己，最开始的那个点
                    Self::gen_health_point((0, 1), dungeon[i][j])
                };
                println!("i = {}, j = {}, res = {:?}", i, j, res[j]);
            }
        }

        res[n - 1].1
    }

    fn gen_health_point(prev: (i32, i32), v: i32) -> (i32, i32) {
        let new_blood = v + prev.0;
        let point = if new_blood < 0 {
            // 1 + blood.abs()
            // 1 是因为要保证点数是大于0的
            1 - new_blood
        } else {
            0
        };
        (new_blood, point.max(prev.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_minimum_hp1() {
        let dungenon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
        assert_eq!(Solution::calculate_minimum_hp(dungenon), 7);
    }

    #[test]
    fn test_calculate_minimum_hp2() {
        let dungenon = vec![vec![0]];
        assert_eq!(Solution::calculate_minimum_hp(dungenon), 1);
    }

    #[test]
    fn test_calculate_minimum_hp3() {
        let dungenon = vec![vec![0, 0, 0], vec![1, 1, -1]];
        assert_eq!(Solution::calculate_minimum_hp(dungenon), 1);
    }

    #[test]
    fn test_calculate_minimum_hp4() {
        let dungenon = vec![vec![1, -3, 3], vec![0, -2, 0], vec![-3, -3, -3]];
        assert_eq!(Solution::calculate_minimum_hp(dungenon), 3);
    }
}
