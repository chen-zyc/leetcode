struct Solution;
impl Solution {
    // 题解中没有使用滚动数组优化的动态规划解法。
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let (m, n) = (s1.len(), s2.len());
        if m + n != s3.len() {
            return false;
        }

        // f[i][j] 表示 s1 的前 i 个和 s2 的前 j 个是否能组合成 s3.
        let mut f = vec![vec![false; n + 1]; m + 1];
        f[0][0] = true;

        for i in 0..=m {
            for j in 0..=n {
                // 当 i=0,j=0 时，这里的 p 是 -1，但 p 的类型是 usize，所以我们移到下面再计算。
                // let p = i + j - 1; // 对应的 s3 的下标
                if i > 0 {
                    f[i][j] = f[i - 1][j] && s1[i - 1] == s3[i + j - 1];
                }
                if j > 0 {
                    // 可能上面计算出来了 f[i][j] == true 了，那这里就不用再计算了
                    f[i][j] = f[i][j] || (f[i][j - 1] && s2[j - 1] == s3[i + j - 1]);
                }
            }
        }

        f[m][n]
    }

    // by zhangyuchen. 通过了，但是 用时 804ms，内存 2M。用时过长。
    pub fn is_interleave1(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        Self::is_interleave1_helper(s1.as_bytes(), 0, s2.as_bytes(), 0, s3.as_bytes(), 0)
    }

    fn is_interleave1_helper(
        s1: &[u8],
        i: usize,
        s2: &[u8],
        j: usize,
        s3: &[u8],
        k: usize,
    ) -> bool {
        // 都到达终点了，匹配
        if k == s3.len() {
            return i == s1.len() && j == s2.len();
        }

        if i < s1.len()
            && s3[k] == s1[i]
            && Self::is_interleave1_helper(s1, i + 1, s2, j, s3, k + 1)
        {
            return true;
        }
        if j < s2.len()
            && s3[k] == s2[j]
            && Self::is_interleave1_helper(s1, i, s2, j + 1, s3, k + 1)
        {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_interleave1() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbcbcac".to_owned(),
            ),
            true
        );
    }

    #[test]
    fn test_is_interleave2() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbbaccc".to_owned(),
            ),
            false
        );
    }
}
