struct Solution;
impl Solution {
    // 参考 链接：https://leetcode-cn.com/problems/burst-balloons/solution/rust-ban-yun-pythondai-ma-0ms-22mb-shuang-bai-by-q/
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // drain(..) 会把所有的元素从 nums 中删除并返回一个迭代器。而 chain 是把两个迭代器连接起来。
        let val = (1..=1)
            .chain(nums.drain(..))
            .chain(1..=1)
            .collect::<Vec<i32>>();
        let mut rec = vec![vec![0; n + 2]; n + 2];

        for i in (0..n).rev() {
            for j in (i + 2)..(n + 2) {
                for k in (i + 1)..j {
                    let sum = val[i] * val[k] * val[j];
                    let sum = sum + rec[i][k] + rec[k][j];
                    rec[i][j] = rec[i][j].max(sum);
                }
            }
        }
        rec[0][n + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_coins1() {
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    }
}
