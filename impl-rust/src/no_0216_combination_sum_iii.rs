struct Solution;
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        Self::helper(k, n, 1, &mut vec![], &mut ans);
        ans
    }

    fn helper(k: i32, n: i32, start: i32, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if k == 0 && n == 0 && tmp.len() > 0 {
            ans.push(tmp.clone());
            return;
        }
        for i in start..=9 {
            // 后面的都比 n 大了，不用再计算了。
            if i > n {
                break;
            }
            tmp.push(i);
            Self::helper(k - 1, n - i, i + 1, tmp, ans);
            tmp.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum3() {
        assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1, 2, 4]]);
        assert_eq!(
            Solution::combination_sum3(3, 9),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        );
    }
}
