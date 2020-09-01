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
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut ptr = &mut dummy;
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val != val {
                ptr.next = Some(node);
                ptr = ptr.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_elements() {
        let head = node(ListNode {
            val: 1,
            next: node(ListNode {
                val: 2,
                next: node(ListNode {
                    val: 6,
                    next: node(ListNode {
                        val: 3,
                        next: node(ListNode {
                            val: 4,
                            next: node(ListNode {
                                val: 5,
                                next: node(ListNode { val: 6, next: None }),
                            }),
                        }),
                    }),
                }),
            }),
        });
        let want = node(ListNode {
            val: 1,
            next: node(ListNode {
                val: 2,
                next: node(ListNode {
                    val: 3,
                    next: node(ListNode {
                        val: 4,
                        next: node(ListNode { val: 5, next: None }),
                    }),
                }),
            }),
        });
        assert_eq!(Solution::remove_elements(head, 6), want);
    }

    fn node(n: ListNode) -> Option<Box<ListNode>> {
        Some(Box::new(n))
    }
}
