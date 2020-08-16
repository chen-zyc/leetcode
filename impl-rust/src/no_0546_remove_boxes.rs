struct Solution;
impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        // 最多有 100 种颜色。
        // dp[l][r][k]
        let mut dp = vec![vec![vec![0; 100]; 100]; 100];
        Self::calculate_points(&boxes, 0, boxes.len() as isize - 1, 0, &mut dp)
    }

    fn calculate_points(
        boxes: &Vec<i32>,
        l: usize,
        r: isize,
        mut k: usize,
        dp: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        if l as isize > r {
            return 0;
        }
        let mut r = r as usize;
        // 有缓存了
        if dp[l][r][k] != 0 {
            return dp[l][r][k];
        }
        // 计算后边有几个连续相同的，计算后，[r] 是最左的那个，并且 r 有可能和 l 指向的是同一位置。
        while r > l && boxes[r] == boxes[r - 1] {
            r -= 1;
            k += 1;
        }

        // 情况1：直接把后面连续的那些删除
        dp[l][r][k] =
            Self::calculate_points(boxes, l, r as isize - 1, 0, dp) + ((k + 1) * (k + 1)) as i32;

        // 情况2：先删除中间的某些区间
        for i in l..r {
            if boxes[i] == boxes[r] {
                dp[l][r][k] = dp[l][r][k].max(
                    // k+1 中的 1 是 [r]
                    Self::calculate_points(boxes, l, i as isize, k+1, dp)
                    // 中间的部分删掉，把这个区间的最高分算上
                    + Self::calculate_points(boxes, i+1, r as isize-1, 0, dp),
                )
            }
        }
        dp[l][r][k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_boxes1() {
        assert_eq!(Solution::remove_boxes(vec![1, 3, 2, 2, 2, 3, 4, 3, 1]), 23);
    }
}
