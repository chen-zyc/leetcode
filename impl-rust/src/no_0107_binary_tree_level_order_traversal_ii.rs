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

use std::collections::LinkedList;

impl Solution {
    // 只使用一个队列。
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }
        let mut ans = Vec::new();
        let mut queue = LinkedList::new();
        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            // 使用长度来标记这一层是否遍历完了。
            let n = queue.len();
            let mut level = Vec::new();
            for _i in 0..n {
                let node = queue.pop_front().unwrap();
                let mut node = node.borrow_mut();
                level.push(node.val);
                if let Some(left) = node.left.take() {
                    queue.push_back(left);
                }
                if let Some(right) = node.right.take() {
                    queue.push_back(right);
                }
            }
            ans.push(level);
        }

        ans.reverse();
        ans
    }

    // by zhangyuchen.
    // 我用两个队列来回倒腾。
    pub fn level_order_bottom1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue1 = LinkedList::new();
        let mut queue2 = LinkedList::new();
        queue1.push_back(root);
        let mut ans = Vec::new();

        while !queue1.is_empty() {
            let mut one_level = Vec::new();
            while let Some(node) = queue1.pop_front() {
                if let Some(n) = node {
                    one_level.push(n.borrow().val);
                    queue2.push_back(n.borrow_mut().left.take());
                    queue2.push_back(n.borrow_mut().right.take());
                }
            }
            if !one_level.is_empty() {
                ans.push(one_level);
            }
            let tmp = queue1;
            queue1 = queue2;
            queue2 = tmp;
        }

        ans.reverse();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_order_bottom() {
        let root = node(TreeNode {
            val: 3,
            left: node(TreeNode::new(9)),
            right: node(TreeNode {
                val: 20,
                left: node(TreeNode::new(15)),
                right: node(TreeNode::new(7)),
            }),
        });
        let want = vec![vec![15, 7], vec![9, 20], vec![3]];
        assert_eq!(Solution::level_order_bottom(root), want);
    }

    fn node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(n)))
    }
}
