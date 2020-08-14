// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use std::cmp::{Ord, Ordering, PartialEq};
use std::collections::BinaryHeap;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // 默认是最大堆，这里颠倒顺序，实现最小堆。
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution;
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        let mut ans = Box::new(ListNode::new(0));
        let mut ptr = &mut ans;
        let mut heap = BinaryHeap::new();
        // 把第一列的元素放到堆里。
        for node in lists {
            if let Some(n) = node {
                heap.push(n);
            }
        }
        // 弹出最小的，然后把它剩下的再加入到堆中。
        while let Some(mut node) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(next);
            }
            ptr.next = Some(node);
            ptr = ptr.next.as_mut().unwrap();
        }

        ans.next
    }

    // 这种是利用数组来生成最终的结果，因为不知道怎么在链表后面插入元素。
    pub fn merge_k_lists1(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        // 用数组来暂存结果。
        let mut ans = Vec::new();
        let mut heap = BinaryHeap::new();
        // 把第一列的元素放到堆里。
        for node in lists {
            if let Some(n) = node {
                heap.push(n);
            }
        }
        // 弹出最小的，然后把它剩下的再加入到堆中。
        while let Some(mut node) = heap.pop() {
            println!("弹出：{}", node.val);
            if let Some(next) = node.next.take() {
                heap.push(next);
            }
            ans.push(node);
        }

        // 把数组转换成链表
        let mut root = None;
        for mut node in ans.into_iter().rev() {
            node.next = root;
            root = Some(node);
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_k_lists() {
        let lists = vec![
            node(ListNode {
                val: 1,
                next: node(ListNode {
                    val: 4,
                    next: node(ListNode::new(5)),
                }),
            }),
            node(ListNode {
                val: 1,
                next: node(ListNode {
                    val: 3,
                    next: node(ListNode::new(4)),
                }),
            }),
            node(ListNode {
                val: 2,
                next: node(ListNode { val: 6, next: None }),
            }),
        ];
        let ans = Solution::merge_k_lists(lists);
        let want = node(ListNode {
            val: 1,
            next: node(ListNode {
                val: 1,
                next: node(ListNode {
                    val: 2,
                    next: node(ListNode {
                        val: 3,
                        next: node(ListNode {
                            val: 4,
                            next: node(ListNode {
                                val: 4,
                                next: node(ListNode {
                                    val: 5,
                                    next: node(ListNode::new(6)),
                                }),
                            }),
                        }),
                    }),
                }),
            }),
        });
        assert_eq!(ans, want);
    }

    fn node(n: ListNode) -> Option<Box<ListNode>> {
        Some(Box::new(n))
    }
}
