struct Solution;

use std::cmp::Ordering;

impl Solution {
    // by zhangyuchen.
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        Self::find(&mut nums, k as usize)
    }

    fn find(nums: &mut [i32], k: usize) -> i32 {
        let mut pos = 1; // 第一个大于等于哨兵的位置
        let sentinel = nums[0]; // 哨兵
        for i in 1..nums.len() {
            if nums[i] < sentinel {
                // 小于哨兵的放到哨兵左侧
                let temp = nums[pos];
                nums[pos] = nums[i];
                nums[i] = temp;
                pos += 1;
            }
        }
        // 把哨兵放到它应该在的位置, pos-1 是最靠右的小于哨兵的位置
        let temp = nums[pos - 1];
        nums[pos - 1] = sentinel;
        nums[0] = temp;

        let right_len = nums.len() - pos + 1; // 右边的大小，包含哨兵(即哨兵是第几大)
        println!(
            "nums = {:?}, k = {}, pos = {}, right_len={}",
            nums, k, pos, right_len
        );

        match right_len.cmp(&k) {
            // 正好等于 k，说明哨兵就是第 k 大的数。
            Ordering::Equal => sentinel,
            // [哨兵, len()] 长度大于 k，说明第 k 大的数在右边
            Ordering::Greater => {
                let (_, right) = nums.split_at_mut(pos);
                Self::find(right, k)
            }
            // 说明右边的数都是比较大的，但第 k 大的数在左边呢。
            Ordering::Less => {
                // left 不包含哨兵了
                let (left, _) = nums.split_at_mut(pos - 1);
                Self::find(left, k - right_len)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_largest_example1() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }

    #[test]
    fn test_find_kth_largest_example2() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }

    #[test]
    fn test_find_kth_largest_example3() {
        assert_eq!(Solution::find_kth_largest(vec![5, 2, 4, 1, 3, 6, 0], 4), 3);
    }
}
