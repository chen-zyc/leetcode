struct Solution;
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();

        let (mut left, mut right) = (new_interval[0], new_interval[1]);
        let mut merged = false; // 是否已经把插入区写入到 ans 中
        for interval in intervals {
            if interval[1] < left {
                // 在插入区的左侧，无交集
                ans.push(interval);
            } else if interval[0] > right {
                // 在插入区的右侧，无交集
                if !merged {
                    ans.push(vec![left, right]);
                    merged = true;
                }
                ans.push(interval);
            } else {
                // 与插入区有交集，计算并集
                left = left.min(interval[0]);
                right = right.max(interval[1]);
            }
        }

        if !merged {
            // 不能直接插入 new_interval，因为 [left, right] 可能在循环中已经改变了。
            ans.push(vec![left, right]);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
}
