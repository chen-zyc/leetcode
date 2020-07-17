struct Solution;
use std::cmp::Ordering;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0_i32;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = (left + right) / 2;
            match nums[mid as usize].cmp(&target) {
                Ordering::Equal => return mid,
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid - 1,
            }
        }
        left
    }

    pub fn search_insert1(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
