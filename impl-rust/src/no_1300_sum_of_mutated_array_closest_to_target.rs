struct Solution;

impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut arr = arr;
        arr.sort();

        let n = arr.len();
        let mut prefix = vec![0; n + 1];
        for i in 1..=n {
            prefix[i] = prefix[i - 1] + arr[i - 1];
        }

        let (mut left, mut right, mut ans, mut ans_sum) = (0, arr[n - 1], 0, 0);
        while left <= right {
            let mid = (left + right) / 2;

            // 计算数组和

            // 找到了，但如果有重复的话，idx 是任意一个的下标
            // 没有找到，idx 是应该插入的位置, 0 或者 n 都有可能。
            let idx = arr.binary_search(&mid).unwrap_or_else(|idx| idx);
            let sum = prefix[idx] + mid * (n - idx) as i32;

            if sum <= target {
                ans = mid;
                ans_sum = sum;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        let big_idx = arr.binary_search(&(ans + 1)).unwrap_or_else(|idx| idx);
        let big_sum = prefix[big_idx] + (ans + 1) * (n - big_idx) as i32;
        if (big_sum - target).abs() < (ans_sum - target).abs() {
            return ans + 1;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_best_value() {
        assert_eq!(Solution::find_best_value(vec![4, 9, 3], 10), 3);
        assert_eq!(Solution::find_best_value(vec![2, 3, 5], 10), 5);
        assert_eq!(
            Solution::find_best_value(vec![60864, 25176, 27249, 21296, 20204], 56803),
            11361
        );
    }
}
