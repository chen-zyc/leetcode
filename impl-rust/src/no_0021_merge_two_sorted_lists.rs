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
struct Solution;
impl Solution {
    // from https://leetcode-cn.com/problems/merge-two-sorted-lists/solution/rust-by-tryfor_-23/
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut res = ListNode::new(0);
        let mut ptr = &mut res;

        while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
            if n1.val <= n2.val {
                // n1 是个引用，不能使用 take()...
                // l1 = n1.next.take();

                // 下面没有再使用 n1, 所以 RAII 会把引用的生命周期结束。
                // ！！！ 先把 l1 的所有权转移了，最后再给它一个新的所有权。 ！！！
                ptr.next = l1;
                ptr = ptr.next.as_mut().unwrap();
                l1 = ptr.next.take();
            } else {
                ptr.next = l2;
                ptr = ptr.next.as_mut().unwrap();
                l2 = ptr.next.take();
            }
        }

        // 剩余的
        ptr.next = if l1.is_some() { l1 } else { l2 };

        res.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists1() {
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let want = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(Solution::merge_two_lists(l1, l2), want);
    }
}
