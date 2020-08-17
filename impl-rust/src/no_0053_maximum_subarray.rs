struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // 链接：https://leetcode-cn.com/problems/maximum-subarray/solution/rustdong-tai-gui-hua-by-youyou-v/
        // 把 pre 也放到初始值里了。
        nums.iter()
            .fold((0, std::i32::MIN), |mut acc, &x| {
                acc.0 = x.max(acc.0 + x);
                acc.1 = acc.0.max(acc.1);
                acc
            })
            .1
    }

    pub fn max_sub_array2(nums: Vec<i32>) -> i32 {
        // 链接：https://leetcode-cn.com/problems/maximum-subarray/solution/rustdong-tai-gui-hua-by-youyou-v/
        let mut pre = 0;
        // 使用 acc 来存储最大值。
        nums.iter().fold(std::i32::MIN, |acc, &x| {
            pre = x.max(pre + x);
            pre.max(acc)
        })
    }

    // 动态规则 + 滚动数组
    pub fn max_sub_array1(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut ans = nums[0];
        let n = nums.len();
        for i in 1..n {
            if nums[i] + nums[i - 1] > nums[i] {
                nums[i] += nums[i - 1];
            }
            ans = ans.max(nums[i]);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array1() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }
}
