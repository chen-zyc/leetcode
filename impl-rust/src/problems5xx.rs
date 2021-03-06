struct Solution;
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![-1; n];
        if n == 0 {
            return ans;
        }

        let mut stack = Vec::new();
        for i in 0..2 * n - 1 {
            let num = nums[i % n];
            while let Some(&top) = stack.last() {
                if num > nums[top] {
                    ans[top] = num;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i % n);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greater_elements() {
        let ans = Solution::next_greater_elements(vec![1, 2, 1]);
        assert_eq!(ans, vec![2, -1, 2]);

        let ans = Solution::next_greater_elements(vec![]);
        assert_eq!(ans, vec![]);
    }
}
