struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n == 0 {
            return true;
        }
        // 最远可以到达的位置
        let mut max_dis = 0;
        for (i, num) in nums.into_iter().enumerate() {
            // 这个位置超过了最远可以到达的位置，后面都不可达了。
            if i > max_dis {
                break;
            }
            max_dis = max_dis.max(i + num as usize);
            // 最远距离已经可以到达最后的位置了。
            if max_dis >= n - 1 {
                return true;
            }
        }

        max_dis >= n - 1
    }

    // 这个实现有点慢。
    pub fn can_jump_slow(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n == 0 {
            return true;
        }
        // mark[i] 表示从 i 出发是否可以到最后一个位置。
        let mut mark = vec![false; n];
        mark[n - 1] = true; // 自己可以到自己。

        for i in (0..n - 1).rev() {
            // 从当前位置到最后一个位置的最远距离。
            let mut max_dis = (nums[i] as usize).min(n - 1 - i);
            // 如果从 i 经过 max_dis 到达的位置标记是 true，那么说明从 i 也可以到达最后的位置。
            while max_dis > 0 {
                if mark[i + max_dis] {
                    mark[i] = true;
                    break;
                }
                max_dis -= 1;
            }
        }

        mark[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
