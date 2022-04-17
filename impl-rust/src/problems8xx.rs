
struct Solution;
impl Solution {
    /// 819. 最常见的单词
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        use std::iter::FromIterator;
        // 转换成 HashSet，方便查找。
        let banned: std::collections::HashSet<String> =
            std::collections::HashSet::from_iter(banned);
        // 记录每个单词出现的次数。
        let mut counter = std::collections::HashMap::new();
        paragraph
            // 分割成单词。
            .split_terminator(&[' ', ',', '!', '?', '\'', ';', '.'][..])
            // 过滤掉空的。
            .filter(|s| !s.is_empty())
            // 转换成小写。
            .map(|s| s.to_ascii_lowercase())
            // 过滤掉在 banned 中的。
            .filter(|s| !banned.contains(s))
            // 统计每个单词出现的次数。
            .for_each(|s| {
                counter.entry(s).and_modify(|v| *v += 1).or_insert(1);
            });
        // 选取出现次数最大的那个单词
        counter
            .iter()
            .max_by(|v1, v2| v1.1.cmp(v2.1))
            .unwrap()
            .0
            .to_string()
    }

    // #867: https://leetcode-cn.com/problems/transpose-matrix/
    // 按对角线交换。
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 这种方式只能交换 m == n 的矩阵，如果 m != n，就不行了。
        // let m = matrix.len(); // 1 <= m,n <= 1000
        // for row in 1..m {
        //     for col in 0..row {
        //         let tmp = matrix[col][row];
        //         matrix[col][row] = matrix[row][col];
        //         matrix[row][col] = tmp;
        //     }
        // }
        // matrix

        // 还是老老实实用复制吧。
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut new_matrix = vec![vec![0; m]; n];
        for (i, row) in matrix.into_iter().enumerate() {
            for (j, cell) in row.into_iter().enumerate() {
                new_matrix[j][i] = cell;
            }
        }
        new_matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let matrix = vec![vec![2, 4, -1], vec![-10, 5, 11], vec![18, -7, 6]];
        let matrix = Solution::transpose(matrix);
        let want = vec![vec![2, -10, 18], vec![4, 5, -7], vec![-1, 11, 6]];
        assert_eq!(matrix, want);

        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let want = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(Solution::transpose(matrix), want);
    }

    #[test]
    fn test_most_common_word() {
        let p = "Bob hit a ball, the hit BALL flew far after it was hit.".to_owned();
        let banned = vec!["hit".to_owned()];
        assert_eq!(Solution::most_common_word(p, banned), "ball".to_owned());
    }
}
