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

struct Solution;
impl Solution {
    // 300. 最长递增子序列
    // https://leetcode-cn.com/problems/longest-increasing-subsequence/
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // 方法1：动态规划。(64ms, 2.2MB)
        // if nums.is_empty() {
        //     return 0;
        // }
        // // dp[i] 表示子序列最后一位是 nums[i] 时的长度。
        // // 默认是1，因为总是包含一位(nums[i] 自身)。
        // let mut dp = vec![1; nums.len()];
        // let mut max_len = 1;
        // for i in 1..nums.len() {
        //     // 在 i 之前找一个比 nums[i] 小的数，则可以把 num 插入到该数后面。
        //     for j in 0..i {
        //         if nums[j] < nums[i] {
        //             dp[i] = dp[i].max(dp[j] + 1);
        //         }
        //     }
        //     max_len = max_len.max(dp[i]);
        // }
        //
        // max_len
        // 方法2：贪心 + 二分(0ms, 2.2MB)
        if nums.is_empty() {
            return 0;
        }

        // d[i] 表示子序列长度为 i 时，子序列中最后那个数中的最小的那个(长度为 i 的子序列可能有多个)。
        let mut d = vec![0; nums.len() + 1];
        let mut len = 1;
        // 长度为 1 时，先假设是 nums[0]，后面会再更新。
        d[len] = nums[0];
        for i in 1..nums.len() {
            if nums[i] > d[len] {
                // 有点大，加入到末尾。
                d[len + 1] = nums[i];
                len += 1;
                continue;
            }
            // 有点小了，从 d 中找到一个比 nums[i] 小的最大的数更新下。
            // 二分查找，l 为实，r 为虚。
            let (mut l, mut r, mut pos) = (1, len, 0);
            while l <= r {
                let mid = l + (r - l) / 2; // 偏右
                if d[mid] >= nums[i] {
                    r = mid - 1;
                } else {
                    pos = mid;
                    l = mid + 1;
                }
            }
            d[pos + 1] = nums[i]; // d[pos+1] 是比 nums[i] 大于或等于的。
        }

        len as i32
    }

    // 338. 比特位计数
    // https://leetcode-cn.com/problems/counting-bits/
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut bits = vec![0; num as usize + 1];

        for i in 1..=num {
            // 如果 i 是偶数，最后一位是 0，那么 [i] == [i >> 1]
            // 如果 i 是奇数，最后一位是 1，那么 [i] == [i >> 1] + 1
            // i & 1 要加扩号，不然会先算 + 再算 &。
            bits[i as usize] = bits[i as usize >> 1] + (i & 1);
        }

        bits
    }

    // 354. 俄罗斯套娃信封问题
    // https://leetcode-cn.com/problems/russian-doll-envelopes/
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        // w 按升序，h 按降序。
        envelopes.sort_unstable_by(|a, b| match a[0].cmp(&b[0]) {
            std::cmp::Ordering::Equal => b[1].cmp(&a[1]), // [1] 按降序
            other => other,
        });

        // f[i] 表示长度为 i 的子序列中最后一个数里最小的那个
        let mut f = Vec::new();
        for envelope in envelopes {
            match f.binary_search(&envelope[1]) {
                Err(idx) => {
                    if idx < f.len() {
                        f[idx] = envelope[1];
                    } else {
                        f.push(envelope[1]);
                    }
                }
                Ok(_) => {}
            }
        }
        f.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits() {
        let ans = Solution::count_bits(2);
        assert_eq!(ans, vec![0, 1, 1]);

        let ans = Solution::count_bits(5);
        assert_eq!(ans, vec![0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn test_length_of_lis() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }

    #[test]
    fn test_max_envelopes() {
        let envelopes = vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]];
        assert_eq!(Solution::max_envelopes(envelopes), 3);
    }
}
