pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort();

    let n = nums.len();
    let mut best_sum = std::i32::MAX;
    let mut best_abs = best_sum;

    for i in 0..n {
        // 去掉重复的
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        // 双指针
        let (mut j, mut k) = (i + 1, n - 1);
        while j < k {
            let sum = nums[i] + nums[j] + nums[k];
            println!("i={}, j={}, k={}, sum={}", i, j, k, sum);
            let abs = (sum - target).abs();
            // 正好等于 target，由于题目保证有唯一解，可以直接返回。
            if abs == 0 {
                return target;
            }
            // 更新最小绝对值。
            if abs < best_abs {
                best_sum = sum;
                best_abs = abs;
            }
            // 和太大了，缩小右指针
            if sum > target {
                // 向前遍历，找到第一个不相同的
                let c = nums[k];
                for k0 in (j..k).rev() {
                    k = k0;
                    if nums[k0] != c {
                        break;
                    }
                }
            }
            // 和太小了，扩大左指针
            if sum < target {
                // 向后遍历，找到第一个不相同的
                let b = nums[j];
                for j0 in j + 1..=k {
                    j = j0;
                    if nums[j0] != b {
                        break;
                    }
                }
            }
        }
    }
    best_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_closest() {
        let ans = three_sum_closest(vec![-1, 2, 1, -4], 1);
        assert_eq!(ans, 2);
    }
}
