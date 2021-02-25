struct Solution;
impl Solution {
    // #867: https://leetcode-cn.com/problems/transpose-matrix/
    // 按对角线交换。
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 这种方式只能交换 m == n 的矩阵，如果 m != n，就不行了。
        // let m = matrix.len(); // 1 <= m,n <= 1000
        // for row in 1..m {
        //     for col in 0..row {
        //         let tmp = matrix[col][row];
        //         matrix[col][row] = matrix[row][col];
        //         matrix[row][col] = tmp;
        //     }
        // }
        // matrix

        // 还是老老实实用复制吧。
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut new_matrix = vec![vec![0; m]; n];
        for (i, row) in matrix.into_iter().enumerate() {
            for (j, cell) in row.into_iter().enumerate() {
                new_matrix[j][i] = cell;
            }
        }
        new_matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let matrix = vec![vec![2, 4, -1], vec![-10, 5, 11], vec![18, -7, 6]];
        let matrix = Solution::transpose(matrix);
        let want = vec![vec![2, -10, 18], vec![4, 5, -7], vec![-1, 11, 6]];
        assert_eq!(matrix, want);

        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let want = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(Solution::transpose(matrix), want);
    }
}
