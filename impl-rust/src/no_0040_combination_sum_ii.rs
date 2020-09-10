struct Solution;
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut ans = Vec::new();
        Self::helper(&candidates, 0, target, &mut vec![], &mut ans);
        ans
    }

    fn helper(
        candidates: &Vec<i32>,
        idx: usize,
        target: i32,
        tmp: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 && tmp.len() > 0 {
            ans.push(tmp.clone());
            return;
        }
        for i in idx..candidates.len() {
            // 保证递归时 target >= 0.
            if target < candidates[i] {
                break;
            }
            // 如果和前面的元素重复了，那么跳过它。这是避免出现重复的组合。
            // 注意这里是 i > idx，不是 i > 0. 如果上一层选择了 1，那么这一层还是可以选择另一个 1.
            // 只是在这一层如果我们选择了 1，下一个还是 1 的话那就不用选 1 了，因为递归时已经选择了第二个 1.
            if i > idx && candidates[i] == candidates[i - 1] {
                continue;
            }
            tmp.push(candidates[i]);
            // idx + 1 保证不重复选择相同的元素
            Self::helper(candidates, i + 1, target - candidates[i], tmp, ans);
            tmp.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum2_1() {
        let ans = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
        let want = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
        assert_eq!(ans, want);
    }

    #[test]
    fn test_combination_sum2_2() {
        let ans = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5);
        let want = vec![vec![1, 2, 2], vec![5]];
        assert_eq!(ans, want);
    }
}
