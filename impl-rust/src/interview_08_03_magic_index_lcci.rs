struct Solution;
impl Solution {
    pub fn find_magic_index(nums: Vec<i32>) -> i32 {
        for (idx, val) in nums.iter().enumerate() {
            if idx as i32 == *val {
                return idx as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_magic_index1() {
        assert_eq!(Solution::find_magic_index(vec![0, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn test_find_magic_index2() {
        assert_eq!(Solution::find_magic_index(vec![1, 1, 1]), 1);
    }
}
