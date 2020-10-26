struct Solution;
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut cnt = vec![0; 101]; // num 最大为 100

        // 每个数字出现的次数
        for num in nums.iter() {
            cnt[*num as usize] += 1;
        }
        // <= num 的数字个数
        for i in 1..cnt.len() {
            cnt[i] += cnt[i - 1];
        }
        let mut ans = vec![0; nums.len()];
        for (i, num) in nums.into_iter().enumerate() {
            // 题目中 num >= 0，所以没有比 0 小的数，保持 0 就好。
            if num > 0 {
                ans[i] = cnt[num as usize - 1];
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smaller_numbers_than_current() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }
}
