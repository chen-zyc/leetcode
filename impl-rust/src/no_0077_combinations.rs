struct Solution;
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        Self::helper(n, k, 1, &mut Vec::with_capacity(k as usize), &mut ans);
        ans
    }

    fn helper(n: i32, k: i32, next: i32, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if k == 0 {
            ans.push(tmp.clone());
            return;
        }
        // [next, n] 中已经不够 k 个了
        if n - next + 1 < k {
            return;
        }
        for i in next..=n {
            tmp.push(i);
            Self::helper(n, k - 1, i + 1, tmp, ans);
            tmp.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine() {
        let mut ans = Solution::combine(4, 2);
        let mut want = vec![
            vec![2, 4],
            vec![3, 4],
            vec![2, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
        ];
        ans.sort_by(|a, b| {
            for (i, n) in a.iter().enumerate() {
                if *n != b[i] {
                    return n.cmp(&b[i]);
                }
            }
            return std::cmp::Ordering::Equal;
        });
        want.sort_by(|a, b| {
            for (i, n) in a.iter().enumerate() {
                if *n != b[i] {
                    return n.cmp(&b[i]);
                }
            }
            return std::cmp::Ordering::Equal;
        });
        assert_eq!(ans, want);
    }
}
