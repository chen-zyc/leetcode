struct Solution;
impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        Self::helper(&mut nums, 0, &mut vec![], &mut ans);
        ans
    }

    fn helper(nums: &mut Vec<i32>, begin: usize, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if begin == nums.len() && tmp.len() > 0 {
            ans.push(tmp.clone());
            return;
        }
        for i in begin..nums.len() {
            tmp.push(nums[i]);
            // 这一层循环中，我们使用到的数都放在 begin 的位置，下一层循环都是从 begin+1 开始。
            nums.swap(i, begin);
            Self::helper(nums, begin + 1, tmp, ans);
            nums.swap(i, begin);
            tmp.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        let ans = Solution::permute(vec![1, 2, 3]);
        let want = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 2, 1],
            vec![3, 1, 2],
        ];
        assert_eq!(ans, want);
    }
}
