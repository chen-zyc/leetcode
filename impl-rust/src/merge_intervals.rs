pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut merged: Vec<Vec<i32>> = Vec::new();
    for interval in intervals {
        // 如果是第一个，或和最后一个不相交，就放到 merged 中
        if merged.is_empty() || merged.last().unwrap()[1] < interval[0] {
            merged.push(interval);
        } else {
            // 和最后一个相交，两个取并集
            merged
                .last_mut()
                .map(|last| last[1] = last[1].max(interval[1]));
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let r = merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]);
        let expect = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(r, expect);

        let r = merge(vec![vec![1, 4], vec![4, 5]]);
        let expect = vec![vec![1, 5]];
        assert_eq!(r, expect);
    }
}
