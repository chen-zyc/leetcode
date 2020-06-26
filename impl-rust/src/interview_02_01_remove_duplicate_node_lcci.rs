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

use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicate_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = ListNode::new(0);
        let mut ptr = &mut new_head;
        let mut set = HashSet::new();
        while let Some(mut node) = head {
            let val = node.val;
            if !set.contains(&val) {
                ptr.next = Some(Box::new(ListNode::new(val)));
                ptr = ptr.next.as_mut().unwrap();
                set.insert(val);
            }
            head = node.next.take();
        }
        new_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut ptr = &mut head;
        for v in values {
            ptr.next = Some(Box::new(ListNode::new(v)));
            ptr = ptr.next.as_mut().unwrap();
        }
        head.next
    }

    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(mut node) = head {
            vec.push(node.val);
            head = node.next.take();
        }
        vec
    }

    #[test]
    fn test_remove_duplicate_nodes_example1() {
        let head = build_list(vec![1, 2, 3, 3, 2, 1]);
        let head = Solution::remove_duplicate_nodes(head);
        assert_eq!(list_to_vec(head), vec![1, 2, 3]);
    }

    #[test]
    fn test_remove_duplicate_nodes_example2() {
        let head = build_list(vec![1, 1, 1, 1, 2]);
        let head = Solution::remove_duplicate_nodes(head);
        assert_eq!(list_to_vec(head), vec![1, 2]);
    }
}
