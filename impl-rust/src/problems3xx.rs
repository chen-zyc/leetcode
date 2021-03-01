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
