pub fn min_distance(word1: String, word2: String) -> i32 {
    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();

    let (m, n) = (word1.len(), word2.len());
    // m 和 n 中有一个为 0
    if m * n == 0 {
        return (m + n) as i32;
    }

    let mut d = vec![vec![0; n + 1]; m + 1];
    // 初始化 [0][j] 和 [i][0]
    for j in 0..n + 1 {
        d[0][j] = j;
    }
    for i in 0..m + 1 {
        d[i][0] = i;
    }

    for i in 1..m + 1 {
        for j in 1..n + 1 {
            // let left = d[i - 1][j] + 1;
            // let top = d[i][j - 1] + 1;
            // // d[i][j] 对应的字符串应该是 s[i-1], s[j-1]，因为 0 在 d 中被当作长度，但在 s 中是第一个
            // let left_top = if word1[i - 1] == word2[j - 1] {
            //     d[i - 1][j - 1]
            // } else {
            //     d[i - 1][j - 1] + 1
            // };
            // d[i][j] = left.min(top.min(left_top));

            // 看别人的答案。
            // 不声明变量可以减少执行时间？
            // min 可以级联这个确实是没想到。
            d[i][j] = if word1[i - 1] == word2[j - 1] {
                (d[i - 1][j] + 1).min(d[i][j - 1] + 1).min(d[i - 1][j - 1])
            } else {
                (d[i - 1][j] + 1)
                    .min(d[i][j - 1] + 1)
                    .min(d[i - 1][j - 1] + 1)
            };
        }
    }

    d[m][n] as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_distance() {
        assert_eq!(min_distance("horse".to_owned(), "ros".to_owned()), 3);
        assert_eq!(
            min_distance("intention".to_owned(), "execution".to_owned()),
            5
        );
    }
}
