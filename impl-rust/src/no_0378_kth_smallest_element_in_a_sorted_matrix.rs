struct Solution;
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let num_rows = matrix.len();
        let (mut min, mut max) = (matrix[0][0], matrix[num_rows - 1][num_rows - 1]); // 左上角和右下角
        while min < max {
            let mid = (max - min) / 2 + min;
            println!("min = {}, max = {}, mid = {}", min, max, mid);
            if Self::count_lte_k(&matrix, mid, num_rows as i32) >= k {
                // 说明答案比 mid 小
                max = mid;
            } else {
                min = mid + 1;
            }
        }
        min
    }

    // 小于等于 mid 的有多少个
    fn count_lte_k(matrix: &Vec<Vec<i32>>, mid: i32, num_rows: i32) -> i32 {
        let mut count = 0;
        let (mut x, mut y) = (num_rows - 1, 0_i32); // 从左下角开始向右上角查找
        while x >= 0 && y < num_rows {
            if matrix[x as usize][y as usize] <= mid {
                count += x + 1; // 这一列有多少个
                y += 1;
            } else {
                x -= 1;
            }
        }
        count
    }

    // by zhangyuchen.
    // 想错了，选中一列后，并不能把矩阵分成两部分，因为左边可能会有数据大于右边，比如：
    // [[1, 2], [4, 5]]
    pub fn kth_smallest_failed(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        // 二分查找
        Self::binary_search(&matrix, 0, matrix.len(), k as usize)
    }

    fn binary_search(matrix: &Vec<Vec<i32>>, i: usize, j: usize, k: usize) -> i32 {
        let mid = (i + j) / 2;
        let num_row = matrix.len();
        if i == j - 1 {
            // 只剩一列，在这一列找就可以了，列也是升序，所以直接返回
            return matrix[k - 1][i];
        }
        let left_count = (mid - i + 1) * num_row; // 包含 mid 那一列
        println!(
            "i={},j={},k={},mid={},left_count={}",
            i, j, k, mid, left_count
        );
        if left_count == k {
            // 左边最大的那个,就是最后一行那个
            return matrix[num_row - 1][mid];
        }
        if left_count < k {
            // 在右边找 k-left_count 小的元素
            return Self::binary_search(matrix, mid + 1, j, k - left_count);
        }
        // 在左边找第 k 小的元素
        // mid-1 的话就少了 num_row 个元素了，不能减
        // 如果只剩最后一列了，这里就死循环了。
        Self::binary_search(matrix, i, mid, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_smallest1() {
        let matrix = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
        assert_eq!(Solution::kth_smallest(matrix, 8), 13);
    }

    #[test]
    fn test_kth_smallest2() {
        let matrix = vec![vec![-5]];
        assert_eq!(Solution::kth_smallest(matrix, 1), -5);
    }

    #[test]
    fn test_kth_smallest3() {
        let matrix = vec![vec![1, 2], vec![1, 3]];
        assert_eq!(Solution::kth_smallest(matrix, 4), 3);
    }

    #[test]
    fn test_kth_smallest4() {
        let matrix = vec![vec![1, 2], vec![3, 3]];
        assert_eq!(Solution::kth_smallest(matrix, 2), 2);
    }

    #[test]
    fn test_kth_smallest5() {
        let matrix = vec![vec![1, 10], vec![2, 20]];
        assert_eq!(Solution::kth_smallest(matrix, 3), 10);
    }
}
