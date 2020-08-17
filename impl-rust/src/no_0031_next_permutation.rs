struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // 找到 [i] > [i-1]，这时 [i..] 是递减的
        let n = nums.len();
        let mut i = n - 1;
        while i > 0 && nums[i] <= nums[i - 1] {
            i -= 1;
        }
        // i == 0 说明没有 [i-1] < [i]，这里直接递增排序返回就可以了。
        if i == 0 {
            nums.sort();
            return;
        }

        // 在 [i..] 中找到最小的那个 > [i-1] 的数，和 [i-1] 交换。
        let val = nums[i - 1];
        for j in (i..n).rev() {
            if nums[j] > val {
                nums[i - 1] = nums[j];
                nums[j] = val;
                break;
            }
        }
        // 把 [i..] 排序成升序
        nums[i..].sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_permutation1() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    #[test]
    fn test_next_permutation2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_next_permutation3() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }
}
