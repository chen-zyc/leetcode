struct Solution;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n <= 1 {
            return;
        }
        // next_blue_idx 因为要递减，所以取 n 而不是 n-1，防止溢出。
        let (mut next_red_idx, mut next_blue_idx) = (0 as usize, n);
        let mut i = 0;
        while i < next_blue_idx && i < n {
            match nums[i] {
                0 => {
                    // 红色的
                    nums.swap(next_red_idx, i);
                    next_red_idx += 1;
                    // 前面的已经判断过了，肯定是白色的，所以这里可以跳过。
                    i += 1;
                }
                2 => {
                    // 蓝色的
                    next_blue_idx -= 1;
                    nums.swap(next_blue_idx, i);
                }
                _ => {
                    // 只有当没有发生交换时才递增，如果交换了，那就要再判断一次 i 的值。
                    i += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);

        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);

        let mut nums = vec![0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0]);

        let mut nums = vec![1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![1]);

        let mut nums = vec![1, 2, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }
}
