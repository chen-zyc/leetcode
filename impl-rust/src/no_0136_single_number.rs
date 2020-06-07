pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut n = 0;
    for num in nums {
        n ^= num;
    }
    n
}

pub fn single_number2(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, num| acc ^ num)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn test_single_number2() {
        assert_eq!(single_number2(vec![2, 2, 1]), 1);
        assert_eq!(single_number2(vec![4, 1, 2, 1, 2]), 4);
    }
}
