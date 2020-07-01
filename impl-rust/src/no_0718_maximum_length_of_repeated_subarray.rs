struct Solution;

impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let (m, n) = (a.len(), b.len());
        let mut ans = 0;
        // b[0] 和 a[i] 对齐
        for i in 0..m {
            ans = ans.max(Self::max_length(&a, i, &b, 0, n.min(m - i)));
        }
        // a[0] 和 b[i] 对齐
        for i in 0..n {
            ans = ans.max(Self::max_length(&a, 0, &b, i, m.min(n - i)));
        }
        ans
    }

    fn max_length(a: &[i32], off_a: usize, b: &[i32], off_b: usize, len: usize) -> i32 {
        let mut max = 0;
        let mut ret = 0;
        for i in 0..len {
            if a[i + off_a] == b[i + off_b] {
                max += 1;
                ret = ret.max(max);
            } else {
                max = 0;
            }
        }
        ret
    }

    // by zhangyuchen.
    pub fn find_length1(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let (m, n) = (a.len(), b.len());
        // c[i][0] 表示 a[i] 与空字符串的最长公共子数组的长度。
        let mut c = vec![vec![0; n + 1]; m + 1];
        let mut ans = 0;
        for i in 1..=m {
            for j in 1..=n {
                if a[i - 1] == b[j - 1] {
                    // 相等的话，a[i] 是公共子数组的一部分
                    c[i][j] = c[i - 1][j - 1] + 1;
                    ans = ans.max(c[i][j]);
                } else {
                    // 不相等的话，就不连续了，设置为 0
                    c[i][j] = 0;
                }
            }
        }

        println!("    {:?}", b);
        for (i, row) in c.iter().enumerate() {
            if i > 0 {
                print!("{}", a[i - 1]);
            } else {
                print!(" ")
            }
            println!("{:?}", row);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_length1() {
        assert_eq!(
            Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]),
            3
        );
    }

    #[test]
    fn test_find_length2() {
        assert_eq!(
            Solution::find_length(vec![0, 1, 1, 1, 1], vec![1, 0, 1, 0, 1]),
            2
        );
    }
}
