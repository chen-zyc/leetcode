pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (mut m, mut n) = (nums1.len(), nums2.len());
    let (mut a, mut b) = (nums1, nums2);
    if m >= n {
        let tmp = a;
        a = b;
        b = tmp;
        m = a.len();
        n = b.len();
    }
    let mut min = 0;
    let mut max = m;
    let half = (m + n + 1) / 2;
    while min <= max {
        let i = (min + max) / 2;
        let j = half - i;
        if i < max && b.get(j - 1) > a.get(i) {
            min = i + 1; // i 太小了
        } else if i > min && a.get(i - 1) > b.get(j) {
            max = i - 1; // i 太大了
        } else {
            // i 正好是中位
            let left_max = if i == 0 {
                b.get(j - 1).unwrap()
            } else if j == 0 {
                a.get(i - 1).unwrap()
            } else {
                a.get(i - 1).unwrap().max(b.get(j - 1).unwrap())
            };
            // 如果是奇数，left_max 就是中位数
            if (m + n) % 2 == 1 {
                return *left_max as f64;
            }

            let right_min = if i == m {
                b.get(j).unwrap()
            } else if j == n {
                a.get(i).unwrap()
            } else {
                b.get(j).unwrap().min(a.get(i).unwrap())
            };
            return (left_max + right_min) as f64 / 2.0;
        }
    }

    0.0
}

#[cfg(test)]
mod test {
    use crate::find_median_sorted_arrays::find_median_sorted_arrays;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }
}
