struct Solution;
impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut ans = Vec::new();
        let mut tmp = Vec::new();
        Self::combination_sum_helper(&candidates, target, &mut tmp, &mut ans);
        ans
    }

    fn combination_sum_helper(
        candidates: &[i32],
        target: i32,
        tmp: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            ans.push(tmp.clone());
            return;
        }
        if target > 0 {
            for (idx, num) in candidates.iter().enumerate() {
                if *num <= target {
                    tmp.push(*num);
                    Self::combination_sum_helper(&candidates[idx..], target - *num, tmp, ans);
                    tmp.pop();
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum() {
        let ans = Solution::combination_sum(vec![2, 3, 6, 7], 7);
        let want = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(ans, want);

        let ans = Solution::combination_sum(vec![2, 3, 5], 8);
        let want = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_eq!(ans, want);
    }
}
