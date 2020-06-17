pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
    // 公式 a[i] + a[j] + i - j
    // 变形 a[i] + i + a[j] - j
    // mx = max(a[i] + i)

    let mut mx = a[0] + 0; // 题目保证了 a.len() >= 2
    let mut ans = 0;
    for j in 1..a.len() {
        ans = ans.max(mx + a[j] - j as i32);
        mx = mx.max(a[j] + j as i32);
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_score_sightseeing_pair() {
        assert_eq!(max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]), 11);
    }
}
