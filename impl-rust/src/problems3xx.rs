// 303. 区域和检索 - 数组不可变
// https://leetcode-cn.com/problems/range-sum-query-immutable/
mod problem303 {
    struct NumArray {
        sums: Vec<i32>,
    }

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl NumArray {
        fn new(nums: Vec<i32>) -> Self {
            // sums[i] 表示 0 -> i-1 项的和。sums[0] = 0 表示 0 前面的和。
            // +1 是为了不用对 0 进行特殊处理。比如 sum_range(0, 1) 这样。
            let mut sums = vec![0; nums.len() + 1];
            for (i, n) in nums.into_iter().enumerate() {
                sums[i + 1] = sums[i] + n;
            }
            NumArray { sums }
        }

        fn sum_range(&self, i: i32, j: i32) -> i32 {
            self.sums[j as usize + 1] - self.sums[i as usize]
        }
    }

    /**
     * Your NumArray object will be instantiated and called as such:
     * let obj = NumArray::new(nums);
     * let ret_1: i32 = obj.sum_range(i, j);
     */
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_num_array() {
            let num_array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
            assert_eq!(num_array.sum_range(0, 2), 1);
            assert_eq!(num_array.sum_range(2, 5), -1);
            assert_eq!(num_array.sum_range(0, 5), -3);
        }
    }
}

mod problem304 {
    struct NumMatrix {
        sums: Vec<Vec<i32>>,
    }

    impl NumMatrix {
        fn new(matrix: Vec<Vec<i32>>) -> Self {
            let m = matrix.len();
            if m == 0 {
                return Self {
                    sums: vec![vec![0]],
                };
            }
            let n = matrix[0].len();
            // 在左上角加一个哨兵。
            let mut sums = vec![vec![0; n + 1]; m + 1];
            for (i, row) in matrix.into_iter().enumerate() {
                for (j, cell) in row.into_iter().enumerate() {
                    // 上 + 左 - 左上 + cell.
                    sums[i + 1][j + 1] = sums[i][j + 1] + sums[i + 1][j] - sums[i][j] + cell;
                }
            }
            Self { sums }
        }

        fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
            // 当前 - 上边 - 左边 + 左上
            // 注意是否包含边界。
            let (row1, col1, row2, col2) =
                (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
            self.sums[row2 + 1][col2 + 1] - self.sums[row1][col2 + 1] - self.sums[row2 + 1][col1]
                + self.sums[row1][col1]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_num_matrix() {
            let matrix = vec![
                vec![3, 0, 1, 4, 2],
                vec![5, 6, 3, 2, 1],
                vec![1, 2, 0, 1, 5],
                vec![4, 1, 0, 1, 7],
                vec![1, 0, 3, 0, 5],
            ];
            let m = NumMatrix::new(matrix);
            assert_eq!(m.sum_region(2, 1, 4, 3), 8);
            assert_eq!(m.sum_region(1, 1, 2, 2), 11);
            assert_eq!(m.sum_region(1, 2, 2, 4), 12);
        }
    }
}
