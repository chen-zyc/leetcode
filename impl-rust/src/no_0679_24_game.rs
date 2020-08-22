struct Solution;

const TARGET: f64 = 24.0;
const EPSILON: f64 = 1e-6;
const ADD: usize = 0;
const MULTIPLY: usize = 1;
const SUBTRACT: usize = 2;
const DIVIDE: usize = 3;

impl Solution {
    pub fn judge_point24(nums: Vec<i32>) -> bool {
        let nums = nums.into_iter().map(|n| n as f64).collect::<Vec<f64>>();
        Self::solve(&nums)
    }

    fn solve(nums: &Vec<f64>) -> bool {
        if nums.is_empty() {
            return false;
        }
        if nums.len() == 1 {
            return (nums[0] - TARGET).abs() < EPSILON;
        }
        let n = nums.len();
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                // 除了 i, j 外剩下的。
                let mut remaining = nums
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != i && *idx != j)
                    .map(|(_, n)| *n)
                    .collect::<Vec<f64>>();

                for op in 0..4 {
                    // 加和乘不需要重复计算
                    if op < SUBTRACT && i < j {
                        continue;
                    }
                    match op {
                        ADD => remaining.push(nums[i] + nums[j]),
                        SUBTRACT => remaining.push(nums[i] - nums[j]),
                        MULTIPLY => remaining.push(nums[i] * nums[j]),
                        DIVIDE => {
                            // 除数不能是0.
                            if nums[j].abs() < EPSILON {
                                continue;
                            }
                            remaining.push(nums[i] / nums[j]);
                        }
                        _ => {}
                    }
                    if Self::solve(&remaining) {
                        return true;
                    }
                    remaining.pop();
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judge_point24() {
        assert_eq!(Solution::judge_point24(vec![4, 1, 8, 7]), true);
        assert_eq!(Solution::judge_point24(vec![1, 2, 1, 2]), false);
    }
}
