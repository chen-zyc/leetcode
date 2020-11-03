struct Solution;
impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        let n = a.len();
        if n < 3 {
            return false;
        }

        let mut idx = 0;
        // 递增
        while idx + 1 < n && a[idx] < a[idx + 1] {
            idx += 1;
        }

        // 最高点不能是数组的第一个位置或最后一个位置
        if idx == 0 || idx == n - 1 {
            return false;
        }

        // 递减
        while idx + 1 < n && a[idx] > a[idx + 1] {
            idx += 1;
        }

        idx + 1 == n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mountain_array() {
        assert_eq!(Solution::valid_mountain_array(vec![2, 1]), false);
        assert_eq!(Solution::valid_mountain_array(vec![3, 5, 5]), false);
        assert_eq!(Solution::valid_mountain_array(vec![0, 3, 2, 1]), true);
    }
}
