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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // 广度优先
        let mut stack_p = Vec::new();
        let mut stack_q = Vec::new();
        stack_p.push(p);
        stack_q.push(q);
        while let (Some(node_p), Some(node_q)) = (stack_p.pop(), stack_q.pop()) {
            // 都是 None, 一致
            if node_p.is_none() && node_q.is_none() {
                continue;
            }
            // 都是 Some, 比较值，并把子节点放到栈中
            if let (Some(node1), Some(node2)) = (node_p, node_q) {
                if node1.borrow().val != node2.borrow().val {
                    return false;
                }
                // 把左右子节点都放到栈中
                stack_p.push(node1.borrow_mut().left.take());
                stack_p.push(node1.borrow_mut().right.take());
                stack_q.push(node2.borrow_mut().left.take());
                stack_q.push(node2.borrow_mut().right.take());
                continue;
            }
            // 一个是 None, 一个是 Some.
            return false;
        }

        stack_p.is_empty() && stack_q.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_tree1() {
        let p = node(TreeNode {
            val: 1,
            left: node(TreeNode::new(2)),
            right: node(TreeNode::new(3)),
        });
        let q = node(TreeNode {
            val: 1,
            left: node(TreeNode::new(2)),
            right: node(TreeNode::new(3)),
        });
        assert!(Solution::is_same_tree(p, q));
    }

    #[test]
    fn test_is_same_tree2() {
        let p = node(TreeNode {
            val: 1,
            left: node(TreeNode::new(2)),
            right: None,
        });
        let q = node(TreeNode {
            val: 1,
            left: None,
            right: node(TreeNode::new(2)),
        });
        assert!(!Solution::is_same_tree(p, q));
    }

    #[test]
    fn test_is_same_tree3() {
        let p = node(TreeNode {
            val: 1,
            left: node(TreeNode::new(2)),
            right: node(TreeNode::new(1)),
        });
        let q = node(TreeNode {
            val: 1,
            left: node(TreeNode::new(1)),
            right: node(TreeNode::new(2)),
        });
        assert!(!Solution::is_same_tree(p, q));
    }

    fn node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(n)))
    }
}
