use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g_heap = BinaryHeap::new(); // 胃口
        for v in g {
            g_heap.push(Reverse(v));
        }
        let mut s_heap = BinaryHeap::new(); // 饼干尺寸
        for v in s {
            s_heap.push(Reverse(v));
        }

        let mut ans = 0;
        loop {
            match (g_heap.peek(), s_heap.peek()) {
                (Some(Reverse(v1)), Some(Reverse(v2))) => {
                    if v2 >= v1 {
                        // 饼干满足胃口
                        s_heap.pop();
                        g_heap.pop();
                        ans += 1;
                    } else {
                        // 饼干不满足胃口
                        s_heap.pop();
                    }
                }
                _ => {
                    break;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_content_children() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
            1
        );
        assert_eq!(
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
            2
        );
    }
}
