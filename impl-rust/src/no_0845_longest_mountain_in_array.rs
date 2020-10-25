struct Solution;
impl Solution {
    pub fn longest_mountain(a: Vec<i32>) -> i32 {
        let mut ans = 0;

        let mut left = 0;
        let n = a.len();
        while left + 2 < n {
            // [left, left+1, left+2]，至少3个
            let mut right = left + 1;
            // 如果是上升的趋势，用 right 找山顶
            if a[left] < a[left + 1] {
                while right + 1 < n && a[right] < a[right + 1] {
                    right += 1;
                }
                if right + 1 < n && a[right] > a[right + 1] {
                    // 如果 right 是山顶，再找右边的山脚
                    while right + 1 < n && a[right] > a[right + 1] {
                        right += 1;
                    }
                    // 找到一段
                    ans = ans.max(right - left + 1);
                } else {
                    // 如果 right 不是山顶，left 从 right+1 继续。
                    right += 1;
                }
            }
            left = right;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_mountain() {
        assert_eq!(Solution::longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]), 5);
        assert_eq!(Solution::longest_mountain(vec![2, 2, 2]), 0);
    }
}
