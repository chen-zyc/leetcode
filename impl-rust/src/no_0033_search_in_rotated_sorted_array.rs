struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut m;
        while l <= r {
            m = (l + r) / 2;
            if nums[m] == target {
                return m as i32;
            }

            if nums[l] <= nums[m] {
                // 左边有序
                if target >= nums[l] && target < nums[m] {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else {
                // 右边有序
                if target > nums[m] && target <= nums[r] {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }
}
