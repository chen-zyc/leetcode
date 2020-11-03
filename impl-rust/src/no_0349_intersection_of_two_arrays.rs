struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1_set = HashSet::<i32>::from_iter(nums1.into_iter());
        let mut ans = Vec::new();
        for num in nums2 {
            // 如果在 set 中出现，则放入 ans 中，然后从 set 中删除。
            if nums1_set.remove(&num) {
                ans.push(num);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        );
        assert_eq!(
            Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![9, 4]
        );
    }
}
