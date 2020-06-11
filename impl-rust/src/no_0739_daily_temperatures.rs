struct Solution;

use std::collections::LinkedList;

impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; t.len()];
        let mut stack = LinkedList::new();

        for (i, temperature) in t.iter().enumerate() {
            let mut front = stack.front();
            while let Some(idx) = front {
                // if t[idx] < temperature, then remove front.
                if let Some(should_remove_front) = t.get(*idx).map(|temp| temp < temperature) {
                    if should_remove_front {
                        res[*idx] = (i - *idx) as i32;
                        stack.pop_front();
                    } else {
                        break;
                    }
                }
                front = stack.front();
            }
            stack.push_front(i);
        }

        res
    }

    // 参考：https://leetcode-cn.com/problems/daily-temperatures/solution/rust-dan-diao-stack-by-jack_/
    pub fn daily_temperatures2(t: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; t.len()];
        // 把下标和温度值都存在栈中，所以这种方法比上面那种占用更多内存。
        let mut stack: LinkedList<(usize, i32)> = LinkedList::new();

        for (i, temperature) in t.iter().enumerate() {
            while let Some((idx, temp)) = stack.front() {
                // if t[idx] < temperature, then remove front.
                if temp < temperature {
                    res[*idx] = (i - idx) as i32;
                    stack.pop_front();
                } else {
                    break;
                }
            }
            stack.push_front((i, *temperature));
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_daily_temperatures() {
        let res = Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]);
        assert_eq!(res, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }

    #[test]
    fn test_daily_temperatures2() {
        let res = Solution::daily_temperatures2(vec![73, 74, 75, 71, 69, 72, 76, 73]);
        assert_eq!(res, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }
}
