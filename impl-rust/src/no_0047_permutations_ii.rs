struct Solution;
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ans = Vec::new();
        let mut visit = vec![false; nums.len()];
        Self::helper2(&nums, &mut visit, &mut vec![], &mut ans);
        ans
    }

    fn helper2(
        nums: &Vec<i32>,
        visit: &mut Vec<bool>,
        tmp: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if tmp.len() == nums.len() {
            ans.push(tmp.clone());
            return;
        }

        // 每次都是从0开始的
        for i in 0..nums.len() {
            if visit[i] || (i > 0 && nums[i] == nums[i - 1] && !visit[i - 1]) {
                continue;
            }
            tmp.push(nums[i]);
            visit[i] = true;
            Self::helper2(nums, visit, tmp, ans);
            // 清除，就当还没用过，继续选下一个数。
            visit[i] = false;
            tmp.pop();
        }
    }

    pub fn permute_unique_failed(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ans = Vec::new();
        Self::helper(&mut nums, 0, &mut vec![], &mut ans);
        ans
    }

    fn helper(nums: &mut Vec<i32>, start: usize, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if start >= nums.len() {
            ans.push(tmp.clone());
            return;
        }

        for i in start..nums.len() {
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            // 这种交换的方式不行，因为上面判断 nums[i] == nums[i-1] 的前提条件是 nums 有序。
            // 如果这里交换了，那么可能会导致 nums[start+1..] 不是有序的，上面再用 nums[i] == nums[i-1] 来去重就不行了。
            nums.swap(i, start);
            tmp.push(nums[start]);
            Self::helper(nums, start + 1, tmp, ans);
            tmp.pop();
            nums.swap(i, start);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute_unique1() {
        let ans = Solution::permute_unique(vec![1, 1, 2]);
        let want = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
        assert_eq!(ans, want);
    }

    #[test]
    fn test_permute_unique2() {
        let ans = Solution::permute_unique(vec![0, 1, 0, 0, 9]);
        let want = vec![
            vec![0, 0, 0, 1, 9],
            vec![0, 0, 0, 9, 1],
            vec![0, 0, 1, 0, 9],
            vec![0, 0, 1, 9, 0],
            vec![0, 0, 9, 0, 1],
            vec![0, 0, 9, 1, 0],
            vec![0, 1, 0, 0, 9],
            vec![0, 1, 0, 9, 0],
            vec![0, 1, 9, 0, 0],
            vec![0, 9, 0, 0, 1],
            vec![0, 9, 0, 1, 0],
            vec![0, 9, 1, 0, 0],
            vec![1, 0, 0, 0, 9],
            vec![1, 0, 0, 9, 0],
            vec![1, 0, 9, 0, 0],
            vec![1, 9, 0, 0, 0],
            vec![9, 0, 0, 0, 1],
            vec![9, 0, 0, 1, 0],
            vec![9, 0, 1, 0, 0],
            vec![9, 1, 0, 0, 0],
        ];
        assert_eq!(ans, want);
    }
}
