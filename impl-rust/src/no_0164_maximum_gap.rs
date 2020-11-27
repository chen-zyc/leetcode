struct Solution;
impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }
        let max = *(nums.iter().max().unwrap());

        // for i:=1; i<=max; i*=10
        let mut buf = vec![0; n];
        for exp in (0..).map(|i| 10_i32.pow(i)).take_while(|i| i <= &max) {
            let mut cnt = vec![0; 10];
            for num in nums.iter() {
                let digit = num / exp % 10;
                cnt[digit as usize] += 1;
            }
            for i in 1..10 {
                cnt[i] += cnt[i - 1];
            }
            for i in (0..n).rev() {
                let digit = (nums[i] / exp % 10) as usize;
                buf[cnt[digit] - 1] = nums[i];
                cnt[digit] -= 1;
            }
            nums.copy_from_slice(&buf);
        }

        let mut ans = 0;
        for i in 1..n {
            ans = ans.max(nums[i] - nums[i - 1]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_gap() {
        assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
        assert_eq!(Solution::maximum_gap(vec![10]), 0);
        assert_eq!(Solution::maximum_gap(vec![100, 3, 2, 1]), 97);
    }
}
