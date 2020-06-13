struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        while left < right {
            let cur_area = (height[left].min(height[right]) as usize) * (right - left);
            if cur_area > area {
                area = cur_area;
            }
            match height[left].cmp(&height[right]) {
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
                Ordering::Equal => {
                    left += 1;
                    right -= 1;
                }
            }
        }
        area as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
