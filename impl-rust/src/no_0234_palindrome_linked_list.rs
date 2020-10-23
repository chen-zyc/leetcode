use crate::common::ListNode;

struct Solution;
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }
        let (mut prev, mut next) = Self::split_list(head);
        // 判断是否是回文
        while let Some(p) = prev {
            if p.val != next.as_ref().unwrap().val {
                return false;
            }
            prev = p.next;
            next = next.unwrap().next;
        }
        true
    }

    fn split_list(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut fast: *const _ = &head;
        let mut slow = head;
        let mut prev = None;
        let mut odd = false; // 节点个数是否是奇数个

        while let Some(n1) = unsafe { &*fast } {
            fast = &n1.next;
            if let Some(n2) = unsafe { &*fast } {
                fast = &n2.next;
                // 把 slow 摘下来放到 prev 链上去
                let slow_next = slow.as_mut().unwrap().next.take();
                slow.as_mut().unwrap().next = prev;
                prev = slow;
                // slow 后移
                slow = slow_next;
            } else {
                // fast 无法跳两个节点时，说明节点个数是奇数个。
                odd = true;
            }
        }

        // prev 链是前半部分反转后的链表，slow 是后半部分未反转的链表
        // slow 可能会比 prev 多一个
        if odd {
            // 如果是奇数个，slow 是中间那个节点，这个节点不用参与比较。
            (prev, slow.unwrap().next)
        } else {
            (prev, slow)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(
            Solution::is_palindrome(ListNode::new_from_arr(&[1, 2])),
            false
        );
        assert_eq!(
            Solution::is_palindrome(ListNode::new_from_arr(&[1, 2, 2, 1])),
            true
        );
        assert_eq!(
            Solution::is_palindrome(ListNode::new_from_arr(&[1, 0, 1])),
            true
        );
    }
}
