struct Solution;
impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        if n <= 1 {
            return false;
        }
        let n = n as usize;
        // f[i] 表示当 n = i 时先手是不是必胜。
        let mut f = vec![false; n + 1];
        f[1] = false;
        f[2] = true; // 要保证 n + 1 > 2
        for i in 3..=n {
            for j in 1..i {
                // 只有 Bob 必败时，Alice 必胜
                if i % j == 0 && !f[i - j] {
                    // 当 n = i 时，Alice 选择 j，那么 Bob 必败。
                    f[i] = true;
                    break;
                }
            }
        }
        f[n]
    }

    // by zhangyuchen.
    // 这种算出来的是 Alice 有没有可能赢。但题目中说“两人都处于最佳状态”，这个意思是说要保证 Alice 必胜 😑。
    pub fn divisor_game_failed(n: i32) -> bool {
        let mut mem = vec![None; n as usize + 1];
        Self::divisor_game_helper(n, &mut mem)
    }

    fn divisor_game_helper(n: i32, mem: &mut Vec<Option<bool>>) -> bool {
        println!("select {}", n);
        if n <= 1 {
            return false;
        }
        if let Some(res) = mem[n as usize] {
            println!("\t当爱丽丝选择 {} 时，答案是 {}", n, res);
            return res;
        }
        for i in 1..n {
            if n % i != 0 {
                continue;
            }
            println!("\t爱丽丝选择了 {}", i);
            let m = n - i;
            let mut bob_selected = false;
            for j in 1..m {
                if m % j != 0 {
                    continue;
                }
                println!("\t鲍勃选择了 {}", j);
                bob_selected = true;
                if Self::divisor_game_helper(m - j, mem) {
                    return true;
                }
            }
            if !bob_selected {
                println!("\t鲍勃遇到了 {}, 没得选", m);
                mem[i as usize] = Some(true);
                return true;
            }
        }
        mem[n as usize] = Some(false);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisor_game1() {
        assert_eq!(Solution::divisor_game(2), true);
    }

    #[test]
    fn test_divisor_game2() {
        assert_eq!(Solution::divisor_game(3), false);
    }

    #[test]
    fn test_divisor_game3() {
        assert_eq!(Solution::divisor_game(5), false);
    }

    #[test]
    fn test_divisor_game4() {
        assert_eq!(Solution::divisor_game(1), false);
    }
}
