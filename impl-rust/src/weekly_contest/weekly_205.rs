struct Solution;
impl Solution {
    // 5507. 替换所有的问号
    pub fn modify_string(mut s: String) -> String {
        const QUESTION: u8 = '?' as u8;
        const A: u8 = 'a' as u8;

        let v = unsafe { s.as_mut_vec() };
        let n = v.len();

        let mut prev = None;
        let mut next = None;
        for i in 0..n {
            if v[i] != QUESTION {
                continue;
            }
            if i > 0 {
                prev = Some(v[i - 1]);
            }
            if i + 1 < n {
                next = Some(v[i + 1]);
            }
            for j in 0 + A..26 + A {
                if Some(j) != prev && Some(j) != next {
                    v[i] = j;
                    break;
                }
            }
        }

        s
    }

    // 5508. 数的平方等于两数乘积的方法数
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut ans = 0;

        let m1 = {
            let mut m = HashMap::new();
            for num in nums1.iter() {
                if let Some(p) = (*num as u128).checked_pow(2) {
                    let v = m.entry(p).or_insert(0);
                    *v += 1;
                }
            }
            m
        };

        let m2 = {
            let mut m = HashMap::new();
            for num in nums2.iter() {
                if let Some(p) = (*num as u128).checked_pow(2) {
                    let v = m.entry(p).or_insert(0);
                    *v += 1;
                }
            }
            m
        };

        for k in 1..nums2.len() {
            for j in 0..k {
                if let Some(m) = (nums2[j] as u128).checked_mul(nums2[k] as u128) {
                    if let Some(v) = m1.get(&m) {
                        ans += v;
                    }
                }
            }
        }
        for k in 1..nums1.len() {
            for j in 0..k {
                if let Some(m) = (nums1[j] as u128).checked_mul(nums1[k] as u128) {
                    if let Some(v) = m2.get(&m) {
                        ans += v;
                    }
                }
            }
        }
        ans
    }

    // 5509. 避免重复字母的最小删除成本
    pub fn min_cost(s: String, mut cost: Vec<i32>) -> i32 {
        let mut ans = 0;

        let s = s.as_bytes();
        for i in 1..s.len() {
            if s[i] == s[i - 1] {
                if cost[i] < cost[i - 1] {
                    ans += cost[i];
                    cost[i] = cost[i - 1];
                } else {
                    ans += cost[i - 1];
                }
            }
        }

        ans
    }

    // 5510. 保证图可完全遍历
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let m = {
            // key = (x, y), value = (A，B，都)
            let mut m = HashMap::new();
            for edge in edges.iter() {
                let pos = (edge[1], edge[2]);
                let value = m.entry(pos).or_insert(vec![0, 0, 0]);
                value[edge[0] as usize - 1] += 1;
            }
            m
        };

        let mut ans = 0;
        for j in 2..=n {
            let mut tmp = vec![0; 3];
            for i in 1..j {
                match m.get(&(i, j)) {
                    // 没有连通
                    None => {}
                    Some(v) => {
                        tmp[0] += v[0];
                        tmp[1] += v[1];
                        tmp[2] += v[2];
                    }
                }
            }
            if tmp[2] > 0 {
                ans += tmp[2] - 1 + tmp[0] + tmp[1];
            } else if tmp[0] > 0 && tmp[1] > 0 {
                ans += tmp[0] - 1 + tmp[1] - 1;
            } else {
                return -1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_string() {
        assert_eq!(
            Solution::modify_string("j?qg??b".to_string()),
            "jaqgacb".to_string()
        );
    }

    #[test]
    fn test_num_triplets() {
        // assert_eq!(Solution::num_triplets(vec![7, 4], vec![5, 2, 8, 9]), 1);
        assert_eq!(Solution::num_triplets(vec![1, 1], vec![1, 1, 1]), 9);
        assert_eq!(
            Solution::num_triplets(vec![7, 7, 8, 3], vec![1, 2, 9, 7]),
            2
        );
        assert_eq!(
            Solution::num_triplets(vec![4, 7, 9, 11, 23], vec![3, 5, 1024, 12, 18]),
            0
        );
        assert_eq!(Solution::num_triplets(vec![43024, 99908], vec![1864]), 0);
    }

    #[test]
    fn test_min_cost() {
        assert_eq!(
            Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5]),
            3
        );
        assert_eq!(Solution::min_cost("abc".to_string(), vec![1, 2, 3]), 0);
        assert_eq!(
            Solution::min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 1]),
            2
        );
    }
}
