use crate::common::head_list_node;
use crate::common::ListNode;

struct Solution;
impl Solution {
    // 参考别人的
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        while let Some(mut n1) = head {
            head = n1.next.take(); // 取下 n1
            if let Some(mut n2) = head {
                head = n2.next.take(); // 取下 n2

                n2.next.replace(n1); // n2.next -> n1
                tail.next.replace(n2); // tail.next -> n2
                tail = tail.next.as_mut().unwrap().next.as_mut().unwrap(); // tail = tail.next.next
            } else {
                // 只有一个节点了，挂到 tail 后面。
                tail.next.replace(n1);
            }
        }

        dummy.next
    }

    pub fn swap_pairs1(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut ptr = &mut dummy;

        loop {
            let (first, remain) = head_list_node(head);
            let (second, remain) = head_list_node(remain);
            let should_break = second.is_none();
            if second.is_some() {
                if let Some(p) = ptr {
                    p.next = second;
                    ptr = &mut p.next;
                }
            }
            if first.is_some() {
                if let Some(p) = ptr {
                    p.next = first;
                    ptr = &mut p.next;
                }
            }
            head = remain;
            if should_break {
                break;
            }
        }

        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_pairs() {
        let head = ListNode::new_from_arr(&vec![1, 2, 3, 4]);
        let want = ListNode::new_from_arr(&vec![2, 1, 4, 3]);
        let ans = Solution::swap_pairs(head);
        assert_eq!(ans, want);
    }
}
