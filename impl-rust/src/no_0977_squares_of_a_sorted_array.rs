struct Solution;
impl Solution {
    // 官方题解中的思路
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; a.len()];
        if a.is_empty() {
            return ans;
        }

        // 这个减少了一半的时间。
        if a[0] >= 0 {
            return a.into_iter().map(|v| v * v).collect();
        }

        let (mut l, mut r) = (0, a.len() - 1);
        // 选择 l 和 r 中较大的那个，逆序放入 ans 中。
        for pos in (0..a.len()).rev() {
            if a[l].abs() > a[r].abs() {
                ans[pos] = a[l].pow(2);
                l += 1;
            } else {
                ans[pos] = a[r].pow(2);
                if r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        ans
    }

    pub fn sorted_squares1(a: Vec<i32>) -> Vec<i32> {
        if a.is_empty() {
            return Vec::new();
        }
        if a[0] >= 0 {
            return a.into_iter().map(|v| v * v).collect();
        }
        let n = a.len();
        // 第一个 >= 0 的索引，或者 a.len()
        let r = a.iter().position(|&v| v >= 0);
        let mut r = r.unwrap_or(n);
        let mut l = r as isize - 1;

        let mut ans = Vec::with_capacity(n);
        while l >= 0 && r < a.len() {
            if a[l as usize].abs() < a[r] {
                ans.push(a[l as usize].pow(2));
                l -= 1;
            } else {
                ans.push(a[r].pow(2));
                r += 1;
            }
        }
        if l >= 0 {
            for i in (0..=l as usize).rev() {
                ans.push(a[i].pow(2));
            }
        }
        for i in r..n {
            ans.push(a[i].pow(2));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_squares() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}
