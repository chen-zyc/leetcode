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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut fast: *const _ = &head;
        let mut slow = head.as_mut().unwrap();

        let mut i = 0;
        while let Some(p) = unsafe { &*fast } {
            fast = &p.next;
            if i > n {
                slow = slow.next.as_mut().unwrap();
            }
            i += 1;
        }
        if i == n {
            // 删掉第一个
            return head.unwrap().next;
        }
        slow.next = slow.next.take().unwrap().next;

        head
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn build_list(n: usize) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut ptr = &mut dummy;
        for i in 1..=n {
            let node = ListNode::new(i as i32);
            ptr.next = Some(Box::new(node));
            // as_mut() -> Option<&mut T>
            ptr = ptr.next.as_mut().unwrap();
        }
        dummy.next
    }

    fn list_to_vec(mut nodes: &mut Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = Vec::new();
        while let Some(ref mut node) = nodes {
            res.push(node.val);
            nodes = &mut node.next;
        }
        res
    }

    #[test]
    fn test_remove_nth_from_end() {
        let nodes = build_list(5);
        let mut nodes = Solution::remove_nth_from_end(nodes, 1);
        assert_eq!(list_to_vec(&mut nodes), vec![1, 2, 3, 4]);
    }
}
