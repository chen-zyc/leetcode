struct Solution;
impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let n = n as usize;
        let fractorial = Self::gen_factorial(n);

        k -= 1; // 从 0 开始的

        // 标记是否被用过了
        let mut valid = vec![1; n + 1];

        let mut ans = String::with_capacity(n);

        for i in 1..=n {
            // valid[0] 是没用的，所以 +1
            let mut order = k / fractorial[n - i] + 1;
            for j in 1..=n {
                order -= valid[j];
                if order == 0 {
                    ans.push(('0' as u8 + j as u8) as char);
                    valid[j] = 0;
                    break;
                }
            }
            k %= fractorial[n - i];
        }

        ans
    }

    // 生成 0! -> (n-1)!
    fn gen_factorial(n: usize) -> Vec<i32> {
        let mut f = Vec::with_capacity(n);
        f.push(1_i32);
        for i in 1..n {
            f.push(f[i - 1] * i as i32);
        }
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_permutation() {
        assert_eq!(Solution::get_permutation(3, 3), "213");
        assert_eq!(Solution::get_permutation(4, 9), "2314");
    }
}
