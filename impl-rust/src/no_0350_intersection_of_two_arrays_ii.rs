struct Solution;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // key => num, value => 出现的次数
        let mut map = std::collections::HashMap::with_capacity(nums1.len());
        for num in nums1 {
            let v = map.entry(num).or_insert(0);
            *v += 1;
        }
        let mut ans = Vec::new();
        for num in nums2 {
            if let Some(count) = map.get_mut(&num) {
                if *count > 0 {
                    *count -= 1;
                    ans.push(num);
                }
            }
        }
        ans
    }

    // 题意理解错了
    // 这个解法求的是连续出现的，题目是可以不连续的，而且顺序也无关。
    pub fn intersect_failed(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // 练习下动态规划
        let (m, n) = (nums1.len(), nums2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut max_len = 0;
        let mut pos = (0, 0);

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if nums1[i] == nums2[j] {
                    dp[i][j] = dp[i + 1][j + 1] + 1;
                } else {
                    dp[i][j] = 0;
                }
                if max_len <= dp[i][j] {
                    max_len = dp[i][j];
                    pos = (i, j);
                }
            }
        }

        nums1
            .get(pos.0..(pos.0 + max_len))
            .map(|x| x.iter().map(|x| *x).collect())
            .unwrap_or(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect1() {
        let mut res = Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]);
        res.sort();
        assert_eq!(res, vec![2, 2]);
    }

    #[test]
    fn test_intersect2() {
        let mut res = Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        res.sort();
        assert_eq!(res, vec![4, 9]);
    }

    #[test]
    fn test_intersect3() {
        let mut res = Solution::intersect(vec![1, 2], vec![2, 1]);
        res.sort();
        assert_eq!(res, vec![1, 2]);
    }
}
