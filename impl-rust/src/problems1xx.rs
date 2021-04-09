struct Solution;
impl Solution {
    // 153. 寻找旋转排序数组中的最小值
    // https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array/
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < nums[right] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        nums[left]
    }

    // 154. 寻找旋转排序数组中的最小值 II
    // https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array-ii/
    pub fn find_min_2(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = left + (right - left) / 2;
            match nums[mid].cmp(&nums[right]) {
                std::cmp::Ordering::Less => right = mid,
                std::cmp::Ordering::Greater => left = mid + 1,
                std::cmp::Ordering::Equal => right -= 1,
            }
        }
        nums[left]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }

    #[test]
    fn test_find_min_2() {
        assert_eq!(Solution::find_min_2(vec![1, 3, 5]), 1);
        assert_eq!(Solution::find_min_2(vec![2, 2, 2, 0, 1]), 0);
    }
}
