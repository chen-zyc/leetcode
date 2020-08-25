struct Solution;
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut temp = Vec::new();
        let mut ans = Vec::new();
        Self::dfs(0, std::i32::MIN, &nums, &mut temp, &mut ans);
        ans
    }

    fn dfs(cur: usize, last: i32, nums: &Vec<i32>, temp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if cur == nums.len() {
            if temp.len() >= 2 {
                ans.push(temp.clone());
            }
            return;
        }
        let cur_val = nums[cur];
        if cur_val >= last {
            temp.push(cur_val);
            Self::dfs(cur + 1, cur_val, nums, temp, ans);
            temp.pop();
        }
        if cur_val != last {
            Self::dfs(cur + 1, last, nums, temp, ans);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_subsequences() {
        let ans = Solution::find_subsequences(vec![4, 6, 7, 7]);
        let want = vec![
            vec![4, 6, 7, 7],
            vec![4, 6, 7],
            vec![4, 6],
            vec![4, 7, 7],
            vec![4, 7],
            vec![6, 7, 7],
            vec![6, 7],
            vec![7, 7],
        ];
        assert_eq!(ans, want);
    }
}
