struct Solution;
impl Solution {
    pub fn unique_paths(mut m: i32, mut n: i32) -> i32 {
        if m <= 1 || n <= 1 {
            return 1;
        }
        if m < n {
            // 为了让下面的 row 占用的空间少点。
            std::mem::swap(&mut m, &mut n);
        }

        let n = n as usize;
        // 第一行的路径数，都是1
        let mut row = vec![1; n];
        // 从第二行开始
        for _i in 1..m {
            // 从第二列开始
            for j in 1..n {
                // 左边的加上边的。
                // row[j] = row[j - 1] + row[j];
                row[j] += row[j - 1];
            }
        }

        row[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }
}
