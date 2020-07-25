struct Solution;
impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        // left 是最大值，right 是所有元素的和
        let (mut left, mut right) = nums
            .iter()
            .fold((0, 0), |acc, num| (acc.0.max(*num), acc.1 + num));
        while left < right {
            let mid = left + (right - left) / 2;
            if Self::check(&nums, mid, m) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    fn check(nums: &[i32], mid: i32, m: i32) -> bool {
        // sum 是某个连续子数组的和，cnt 是子数组的和 <= mid 的个数
        let (_, cnt) = nums.iter().fold((0, 1), |acc, num| {
            if acc.0 + num > mid {
                (*num, acc.1 + 1)
            } else {
                (acc.0 + num, acc.1)
            }
        });
        cnt <= m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_array() {
        assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
    }
}
