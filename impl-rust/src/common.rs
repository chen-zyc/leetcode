use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn tree_node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(n)))
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn new_from_arr(arr: &[i32]) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut ptr = &mut head;
        for val in arr {
            if let Some(p) = ptr {
                p.next = Some(Box::new(ListNode::new(*val)));
                ptr = &mut p.next;
            }
        }
        head.unwrap().next
    }
}

pub fn head_list_node(
    head: Option<Box<ListNode>>,
) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    if let Some(mut n) = head {
        let remain = n.next.take();
        return (Some(n), remain);
    }
    (None, None)
}

pub fn sort_2d_vec(v: &mut Vec<Vec<i32>>) {
    v.sort_by(|a, b| match a.len().cmp(&b.len()) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            for i in 0..a.len() {
                if a[i] != b[i] {
                    return a[i].cmp(&b[i]);
                }
            }
            Ordering::Equal
        }
    });
}
