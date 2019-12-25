pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let complement = target - *num;
        if let Some(v) = map.get(&complement) {
            return vec![*v, i as i32];
        }
        map.insert(*num, i as i32);
    }
    vec![]
}

#[cfg(test)]
mod test {
    use crate::two_sum::two_sum;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let sum = two_sum(nums, target);
        assert_eq!(sum, vec![0, 1]);

        // 数组中有重复的数字
        let nums = vec![2, 2, 4, 5, 6];
        let target = 7;
        let sum = two_sum(nums, target);
        assert_eq!(sum, vec![1, 3]);
    }
}
