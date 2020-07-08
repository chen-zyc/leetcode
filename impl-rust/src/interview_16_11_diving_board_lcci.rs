struct Solution;
impl Solution {
    pub fn diving_board(shorter: i32, longer: i32, k: i32) -> Vec<i32> {
        if k == 0 {
            return Vec::new();
        }
        if shorter == longer {
            return vec![shorter * k];
        }

        let mut lengths = Vec::with_capacity(k as usize + 1);

        // i 表示长木板的块数。
        for i in 0..=k {
            lengths.push(shorter * (k - i) + longer * i)
        }

        lengths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diving_board1() {
        assert_eq!(Solution::diving_board(1, 2, 3), vec![3, 4, 5, 6]);
    }
}
