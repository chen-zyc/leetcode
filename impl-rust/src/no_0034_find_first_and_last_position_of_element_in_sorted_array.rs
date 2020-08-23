struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left_idx = Self::extreme_insertion_index(&nums, target, true);
        // 如果没找到
        if left_idx < 0 || left_idx >= nums.len() as i32 || nums[left_idx as usize] != target {
            return vec![-1, -1];
        }
        vec![
            left_idx,
            Self::extreme_insertion_index(&nums, target, false) - 1,
        ]
    }

    fn extreme_insertion_index(nums: &Vec<i32>, target: i32, left: bool) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let m = (l + r) / 2;
            // 主要是 == target 时应该怎么处理。
            // 如果 left == true，那么当 == target 时就把右边的舍去。
            // 如果 left == false, 那么就把左边包括 m 都舍去，其实是在找比 target 大的那个数
            if nums[m] > target || (left && nums[m] == target) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l as i32
    }

    // by zhangyuchen. 这种有可能会退化成 O(n) 复杂度。
    pub fn search_range1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match nums.binary_search(&target) {
            Ok(idx) => {
                let mut left = idx as i32;
                while left >= 0 && nums[left as usize] == target {
                    left -= 1;
                }
                let mut right = idx as i32;
                while right < nums.len() as i32 && nums[right as usize] == target {
                    right += 1;
                }
                vec![left + 1, right - 1]
            }
            Err(_) => vec![-1, -1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_range() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
    }
}
