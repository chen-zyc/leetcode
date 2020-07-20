struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        // 每个值不能超过 target 的一半, +1 是防止 target 是奇数。
        let max_per_val = target / 2 + 1;
        let n = numbers.len();
        for i in 0..n {
            let first = numbers[i];
            // 最小的值都超过 target 的一半了，另一个肯定也超过了，后面就不用选了。
            if first > max_per_val {
                break;
            }
            // 在剩下的区间中找到另一个值
            if let Some(pos) = Self::binary_search(&numbers, i + 1, n - 1, target - first) {
                return vec![i as i32 + 1, pos as i32 + 1];
            }
        }
        vec![0; 2]
    }

    fn binary_search(
        numbers: &Vec<i32>,
        mut left: usize,
        mut right: usize,
        target: i32,
    ) -> Option<usize> {
        while left <= right {
            let mid = (left + right) / 2;
            match numbers[mid].cmp(&target) {
                std::cmp::Ordering::Equal => {
                    return Some(mid);
                }
                std::cmp::Ordering::Less => {
                    left = mid + 1;
                }
                std::cmp::Ordering::Greater => {
                    right = mid - 1;
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }
}
