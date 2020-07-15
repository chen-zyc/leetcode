struct Solution;
impl Solution {
    // by zhangyuchen. 只使用 O(n) 的空间。
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.is_empty() {
            return 0;
        }

        let rows = triangle.len();
        let mut dp = vec![std::i32::MAX; triangle[rows - 1].len()]; // 最长的一列
        dp[0] = triangle[0][0];

        for i in 1..rows {
            let cols = triangle[i].len();
            // 因为要依赖 dp[j-1]，所以如果正序计算的话会覆盖上一层的值。
            for j in (0..cols).rev() {
                // 可以单独计算 j=0 和 j=cols-1 时的场景，避免使用 i32::MAX。
                // 上一层和 [i][j] 相关的是 [i-1][j-1] 和 [i-1][j]
                let top1 = if j > 0 { dp[j - 1] } else { std::i32::MAX };
                // 最后一列是没有 dp[j] 的
                let top2 = if j < cols - 1 { dp[j] } else { std::i32::MAX };
                dp[j] = triangle[i][j] + top1.min(top2);
                println!("dp[{}][{}] = {}", i, j, dp[j]);
            }
        }

        println!("{:?}", dp);
        *dp.iter().min().unwrap()
    }

    // by zhangyuchen.
    pub fn minimum_total1(triangle: Vec<Vec<i32>>) -> i32 {
        // 如果不加 cache，会超出时间限制。
        let mut cache = Vec::new();
        for row in triangle.iter() {
            cache.push(vec![None; row.len()]);
        }
        Self::dp(&triangle, 0, 0, &mut cache)
    }

    fn dp(triangle: &Vec<Vec<i32>>, i: usize, j: usize, cache: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if i >= triangle.len() || j >= triangle[i].len() {
            return 0;
        }
        if let Some(v) = cache[i][j] {
            return v;
        }
        let min = triangle[i][j]
            + Self::dp(triangle, i + 1, j, cache).min(Self::dp(triangle, i + 1, j + 1, cache));
        cache[i][j] = Some(min);
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_total() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(Solution::minimum_total(triangle), 11);
    }
}
