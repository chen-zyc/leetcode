struct Solution;

const CHAR_A_U8: u8 = 'a' as u8;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut ans = Vec::new();

        // 每个字母最后出现的位置。
        let last_pos = s.chars().enumerate().fold([0; 26], |mut acc, (idx, c)| {
            acc[(c as u8 - CHAR_A_U8) as usize] = idx;
            acc
        });

        let (mut start, mut end) = (0, 0);
        for (idx, c) in s.chars().enumerate() {
            // 当前片段中所有字母出现的最后位置。
            end = end.max(last_pos[(c as u8 - CHAR_A_U8) as usize]);

            // 已经遍历到当前片段的右边界了，找到了一个片段。
            if idx == end {
                ans.push((end - start + 1) as i32);
                start = end + 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_labels() {
        assert_eq!(
            Solution::partition_labels("ababcbacadefegdehijhklij".to_owned()),
            vec![9, 7, 8]
        );
    }
}
