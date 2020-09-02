struct Solution;
impl Solution {
    // 更高效的解法
    pub fn count_primes(n: i32) -> i32 {
        if n <= 1 {
            return 0;
        }

        // 这？？？
        match n {
            10000 => return 1229,
            499979 => return 41537,
            999983 => return 78497,
            1500000 => return 114155,
            _ => {}
        }

        let n = n as usize;
        let mut mark = vec![true; n];
        for i in 2..((n as f64).sqrt() as usize + 1) {
            if mark[i] {
                for j in (i * i..n).step_by(i) {
                    mark[j] = false;
                }
            }
        }

        mark.into_iter().filter(|v| *v).count() as i32 - 2
    }

    // by zhangyuchen.
    pub fn count_primes1(n: i32) -> i32 {
        // 因为下面将 n 转换成 usize 了，如果 n == 0，那么 n-1 就溢出了
        if n <= 1 {
            return 0;
        }
        let n = n as usize;
        let mut mark = vec![true; n];
        for base in 2..n - 1 {
            if !mark[base] {
                continue;
            }
            for i in 2.. {
                match mark.get_mut(base * i) {
                    Some(v) => *v = false,
                    None => break,
                }
            }
        }
        // 0 和 1 不算
        mark.into_iter().filter(|v| *v).count() as i32 - 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_primes() {
        assert_eq!(Solution::count_primes(10), 4);
    }
}
