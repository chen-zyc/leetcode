pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 2 {
        return n as i32;
    }

    let mut i = 0;
    for j in 1..n {
        let (cur, pre) = (nums[j], nums[i]);
        if cur != pre {
            i += 1;
            if i != j { // 避免没有必要的赋值
                nums[i] = cur;
            }
        }
    }
    (i + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2);
        assert_eq!(remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]), 5);
        assert_eq!(remove_duplicates(&mut vec![1, 2, 3]), 3);
    }
}