struct Solution;
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x = 0 as i32;
        let mut y = 0 as i32;
        for m in moves.chars() {
            match m {
                'U' => x -= 1,
                'D' => x += 1,
                'L' => y -= 1,
                'R' => y += 1,
                _ => {}
            }
        }
        x == 0 && y == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judge_circle() {
        assert_eq!(Solution::judge_circle("UD".to_string()), true);
        assert_eq!(Solution::judge_circle("LL".to_string()), false);
    }
}
