use crate::common::ListNode;

struct Solution;
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }
        let mid = Self::middle_node(head);
        let mid = Self::reverse_list(mid);
        Self::merge_list(head, mid);
    }

    fn middle_node(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast: *const _ = head;
        let mut slow = head;

        while let Some(node) = unsafe { &*fast } {
            fast = &node.next;
            if let Some(n) = unsafe { &*fast } {
                fast = &n.next;
                slow = &mut slow.as_mut().unwrap().next;
            }
        }

        // 返回中间节点后的列表
        slow.as_mut().unwrap().next.take()
    }

    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut node) = head.take() {
            head = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }

    // 把 l2 合并到 l1.
    fn merge_list(mut l1: &mut Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) {
        while let Some(mut node) = l2.take() {
            l2 = node.next.take();

            if let Some(n) = l1 {
                node.next = n.next.take();
                n.next = Some(node);
                l1 = &mut n.next.as_mut().unwrap().next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reorder_list1() {
        let mut list = ListNode::new_from_arr(&[1, 2, 3, 4]);
        Solution::reorder_list(&mut list);
        let want = ListNode::new_from_arr(&[1, 4, 2, 3]);
        assert_eq!(list, want);
    }

    #[test]
    fn test_reorder_list2() {
        let mut list = ListNode::new_from_arr(&[1, 2, 3, 4, 5]);
        Solution::reorder_list(&mut list);
        let want = ListNode::new_from_arr(&[1, 5, 2, 4, 3]);
        assert_eq!(list, want);
    }
}
