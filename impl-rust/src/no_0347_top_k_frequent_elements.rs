struct Solution;

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq)]
struct Element {
    value: i32,
    rate: u32,
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rate.cmp(&other.rate)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.rate == other.rate
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let m = {
            let mut m = HashMap::with_capacity(k);
            for num in nums.iter() {
                let v = m.entry(num).or_insert(0);
                *v += 1;
            }
            m
        };
        // 最小堆
        let mut heap = BinaryHeap::with_capacity(k + 1);
        for (idx, v) in m.into_iter().enumerate() {
            // 也有的是直接存储的元组，(频率, 值)
            heap.push(Reverse(Element {
                value: *v.0,
                rate: v.1,
            }));
            if idx >= k as usize {
                heap.pop();
            }
        }

        let mut ans = Vec::with_capacity(k);
        while let Some(v) = heap.pop() {
            ans.push(v.0.value);
        }
        ans.reverse();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
