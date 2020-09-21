struct Solution;
impl Solution {
    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        for i in 0..=nums.len() {
            Self::select(&mut nums, i, 0, &mut vec![], &mut ans);
        }
        ans
    }

    fn select(
        nums: &mut Vec<i32>,
        n: usize,
        start: usize,
        tmp: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if n == 0 {
            ans.push(tmp.clone());
            return;
        }
        for i in start..nums.len() {
            nums.swap(i, start);
            tmp.push(nums[start]);
            Self::select(nums, n - 1, i + 1, tmp, ans);
            tmp.pop();
            nums.swap(i, start);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::sort_2d_vec;

    #[test]
    fn test_subsets() {
        let mut ans = Solution::subsets(vec![1, 2, 3]);
        let mut want = vec![
            vec![3],
            vec![1],
            vec![2],
            vec![1, 2, 3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2],
            vec![],
        ];
        sort_2d_vec(&mut ans);
        sort_2d_vec(&mut want);

        assert_eq!(ans, want);
    }
}
