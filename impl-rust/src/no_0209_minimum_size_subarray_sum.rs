struct Solution;
impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        // 滑动窗口
        let mut min_len = nums.len() + 1;
        let (mut window_sum, mut window_len) = (0, 0);
        let (mut left, mut right) = (0, 0);
        while right < nums.len() {
            window_sum += nums[right];
            window_len += 1;
            right += 1;
            println!(
                "收缩前：left = {}, right = {}, window_sum = {}, window_len = {}",
                left, right, window_sum, window_len
            );
            // 还不满足，继续扩张
            if window_sum < s {
                continue;
            }
            // 窗口中的和现在 >= s 了。

            // 现在这个窗口的长度如果是最小的，就更新
            if window_len < min_len {
                min_len = window_len;
            }

            // 增加左指针，直到容器的和不满足 >= s 为止
            while left < right {
                window_sum -= nums[left];
                window_len -= 1;
                left += 1;
                // 收缩后也可能会满足 >= s 的条件，所以也需要和 min_len 比较下
                if window_sum >= s {
                    if window_len < min_len {
                        min_len = window_len;
                    }
                } else {
                    break;
                }
            }
            println!(
                "收缩后：left = {}, right = {}, window_sum = {}, window_len = {}",
                left, right, window_sum, window_len
            );
        }
        if min_len > nums.len() {
            0
        } else {
            min_len as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sub_array_len() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }
}
