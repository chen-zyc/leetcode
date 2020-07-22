struct Solution;
impl Solution {
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        // 使用二分法
        let (mut left, mut right) = (0, numbers.len() - 1);
        while left < right {
            let mid = (left + right) / 2;
            // 如果 mid 比 left 大，说明最小值在 [mid+1, right] 之间，也可能就是 left。
            // 如果 mid 比 left 小，说明最小值在 [left+1, mid] 之间。
            // 如果 mid 比 right 小，说明最小值在 [left, mid] 之间。
            // 如果 mid 比 right 大，说明最小值在 [left, mid] 之间。
            // 上面这个判断不出应该怎么缩小范围。
            // 可以判断 mid 是在较小的那块上还是在较大的那块上吗？
            // 如果 mid > left && mid > right, 说明 mid 是在较大的那块上，否则就是在较小的那块上。
            // 如果 mid 就是最小值呢？那么有：mid < left && mid < right. 但反过来不一定。
            // 还是有点繁琐。
            // 如果 mid 比 right 大，是不是就说明 mid 在较大的那块上了。如果 mid <= right，那说明 mid 在小块上。
            // 如果 mid == right，那么 mid 可能在大块上，也可能在小块上。参考单测4.
            // 看了官方题解，当相等时直接把 right 减一 😳，这样就没有二分了呀。。。
            if numbers[mid] > numbers[right] {
                left = mid + 1;
            } else if numbers[mid] < numbers[right] {
                right = mid;
            } else {
                right -= 1;
            }
        }
        numbers[left]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_array1() {
        assert_eq!(Solution::min_array(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn test_min_array2() {
        assert_eq!(Solution::min_array(vec![2, 2, 2, 0, 1]), 0);
    }

    #[test]
    fn test_min_array3() {
        assert_eq!(Solution::min_array(vec![1, 1]), 1);
    }

    #[test]
    fn test_min_array4() {
        assert_eq!(Solution::min_array(vec![3, 3, 1, 3]), 1);
    }
}
