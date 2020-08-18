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
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    // 分治 + 中序遍历优化
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let head = head.as_ref();
        let len = Self::get_length(head);
        let (_, tree) = Self::build_tree2(head, 0, len - 1);
        tree
    }

    fn build_tree2(
        head: Option<&Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> (Option<&Box<ListNode>>, Option<Rc<RefCell<TreeNode>>>) {
        if left > right {
            return (head, None);
        }

        let mid = (left + right + 1) / 2;
        let (head, left) = Self::build_tree2(head, left, mid - 1);
        if let Some(head) = head {
            let mut root = TreeNode::new(head.val);
            root.left = left;
            let (head, right) = Self::build_tree2(head.next.as_ref(), mid + 1, right);
            root.right = right;
            return (head, Some(Rc::new(RefCell::new(root))));
        }

        (head, None)
    }

    fn get_length(mut head: Option<&Box<ListNode>>) -> i32 {
        let mut len = 0;
        while let Some(node) = head {
            head = node.next.as_ref();
            len += 1;
        }
        len
    }

    // 使用快慢指针来求中间点。
    pub fn sorted_list_to_bst1(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree(head.as_ref(), None)
    }

    fn build_tree(
        left: Option<&Box<ListNode>>,
        right: Option<&Box<ListNode>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if Self::ptr_eq(left, right) {
            return None;
        }
        let mid = Self::get_median(left, right);
        if let Some(mid_node) = mid {
            return Some(Rc::new(RefCell::new(TreeNode {
                val: mid_node.val,
                left: Self::build_tree(left, mid),
                right: Self::build_tree(Self::next_node(mid), right),
            })));
        }
        None
    }

    fn get_median<'a>(
        left: Option<&'a Box<ListNode>>,
        right: Option<&'a Box<ListNode>>,
    ) -> Option<&'a Box<ListNode>> {
        let mut fast = left;
        let mut slow = left;

        // while fast != right && fast.next != right {
        //  fast = fast.next.next;
        //  slow = slow.next;
        // }
        while !Self::ptr_eq(fast, right) && !Self::ptr_eq(Self::next_node(fast), right) {
            fast = Self::next_node(Self::next_node(fast));
            slow = Self::next_node(slow);
        }

        slow
    }

    fn next_node<'a>(n: Option<&'a Box<ListNode>>) -> Option<&'a Box<ListNode>> {
        n.and_then(|n| n.next.as_ref())
    }

    fn ptr_eq(p1: Option<&Box<ListNode>>, p2: Option<&Box<ListNode>>) -> bool {
        match (p1, p2) {
            (None, None) => true,
            (Some(p1), Some(p2)) => std::ptr::eq(p1, p2),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_list_to_bst1() {
        let head = Some(Box::new(ListNode {
            val: -10,
            next: Some(Box::new(ListNode {
                val: -3,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode { val: 9, next: None })),
                    })),
                })),
            })),
        }));

        let ans = Solution::sorted_list_to_bst(head);
        let want = tree_node(TreeNode {
            val: 0,
            left: tree_node(TreeNode {
                val: -3,
                left: tree_node(TreeNode::new(-10)),
                right: None,
            }),
            right: tree_node(TreeNode {
                val: 9,
                left: tree_node(TreeNode::new(5)),
                right: None,
            }),
        });
        assert_eq!(ans, want);
    }

    fn tree_node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(n)))
    }

    #[test]
    fn test_demo() {
        let n1 = Box::new(ListNode::new(1));
        assert!(std::ptr::eq(&n1, &n1));
    }
}
