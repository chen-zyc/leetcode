struct Solution;
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let (rows, columns) = (matrix.len(), matrix[0].len());
        let mut memo = vec![vec![0; columns]; rows];
        let mut ans = 0;
        for i in 0..rows {
            for j in 0..columns {
                ans = ans.max(Self::dfs(&matrix, i, j, &mut memo));
            }
        }
        ans
    }

    fn dfs(matrix: &Vec<Vec<i32>>, row: usize, col: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if memo[row][col] != 0 {
            return memo[row][col];
        }
        memo[row][col] += 1;

        // 上
        if row > 0 && matrix[row - 1][col] > matrix[row][col] {
            memo[row][col] = memo[row][col].max(Self::dfs(matrix, row - 1, col, memo) + 1);
        }
        // 左
        if col > 0 && matrix[row][col - 1] > matrix[row][col] {
            memo[row][col] = memo[row][col].max(Self::dfs(matrix, row, col - 1, memo) + 1);
        }
        // 右
        if col < matrix[0].len() - 1 && matrix[row][col + 1] > matrix[row][col] {
            memo[row][col] = memo[row][col].max(Self::dfs(matrix, row, col + 1, memo) + 1);
        }
        // 下
        if row < matrix.len() - 1 && matrix[row + 1][col] > matrix[row][col] {
            memo[row][col] = memo[row][col].max(Self::dfs(matrix, row + 1, col, memo) + 1);
        }
        memo[row][col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_increasing_path1() {
        let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
        assert_eq!(Solution::longest_increasing_path(matrix), 4);
    }

    #[test]
    fn test_longest_increasing_path2() {
        let matrix = vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]];
        assert_eq!(Solution::longest_increasing_path(matrix), 4);
    }
}
