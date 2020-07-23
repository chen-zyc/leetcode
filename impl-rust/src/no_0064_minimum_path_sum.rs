struct Solution;
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut arr = vec![std::i32::MAX; n];
        arr[0] = 0;
        for i in 0..m {
            for j in 0..n {
                // 选择上面(arr[j])和左面(arr[j-1])最小的那个
                if j == 0 {
                    // 左边没有，只有上面的元素
                    arr[j] = arr[j] + grid[i][j];
                } else {
                    // 有左边和上边
                    arr[j] = arr[j].min(arr[j - 1]) + grid[i][j];
                }
            }
        }

        arr[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(Solution::min_path_sum(grid), 7);
    }
}
