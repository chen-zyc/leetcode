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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if l1.is_none() {
        return l2;
    }
    if l2.is_none() {
        return l1;
    }

    let mut head = None;
    let mut cur_node = &mut head;
    let mut carry = 0;
    let mut l1 = l1;
    let mut l2 = l2;
    while l1.is_some() || l2.is_some() {
        let x = l1.take().map_or(0, |mut node| {
            l1 = node.next.take();
            node.val
        });
        let y = l2.take().map_or(0, |mut node| {
            l2 = node.next.take();
            node.val
        });
        let mut sum = x + y + carry;
        carry = if sum >= 10 {
            sum -= 10;
            1
        } else {
            0
        };
        if cur_node.is_none() {
            head = Some(Box::new(ListNode::new(sum)));
            cur_node = &mut head;
        } else {
            cur_node.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            cur_node = &mut cur_node.as_mut().unwrap().next;
        }
    }

    // 最后可以还有进位没有加入到链表
    if carry > 0 {
        cur_node.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
    }

    head
}

#[cfg(test)]
mod test {
    use crate::add_two_numbers::{add_two_numbers, ListNode};

    // 传入的 v 是正序的，返回的结果是倒序的。
    fn build_list_from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut r = None;
        for n in v.iter() {
            // 每个新的节点都插入到链表开头。
            let mut node = ListNode::new(*n);
            node.next = r.take();
            r = Some(Box::new(node));
        }
        r
    }

    #[test]
    fn test_add_two_numbers() {
        let l1 = build_list_from_vec(vec![3, 4, 2]);
        let l2 = build_list_from_vec(vec![4, 6, 5]);
        assert_eq!(add_two_numbers(l1, l2), build_list_from_vec(vec![8, 0, 7]));

        // 一长一短
        let l1 = build_list_from_vec(vec![1, 0]);
        let l2 = build_list_from_vec(vec![2, 1, 0]);
        assert_eq!(add_two_numbers(l1, l2), build_list_from_vec(vec![2, 2, 0]));

        // 一个为空
        let l1 = build_list_from_vec(vec![]);
        let l2 = build_list_from_vec(vec![1, 0]);
        assert_eq!(add_two_numbers(l1, l2), build_list_from_vec(vec![1, 0]));

        // 运算最后出现进位
        let l1 = build_list_from_vec(vec![9, 9]);
        let l2 = build_list_from_vec(vec![1]);
        assert_eq!(add_two_numbers(l1, l2), build_list_from_vec(vec![1, 0, 0]));
    }
}
