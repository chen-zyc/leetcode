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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return Vec::new();
        }

        let mut ans = Vec::new();
        let mut stack = Vec::new();
        stack.push(root.unwrap());

        while let Some(node) = stack.pop() {
            let left_node = node.borrow_mut().left.take();
            if let Some(left) = left_node {
                // 有左子树，把 node 和左子树都放进去
                stack.push(node);
                stack.push(left);
                continue;
            }

            // 访问中间的节点。
            ans.push(node.borrow().val);

            let right_node = node.borrow_mut().right.take();
            if let Some(right) = right_node {
                // 只有右子节点，把它入栈
                stack.push(right);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inorder_traversal() {
        let root = node(TreeNode {
            val: 1,
            left: None,
            right: node(TreeNode {
                val: 2,
                left: node(TreeNode::new(3)),
                right: None,
            }),
        });
        assert_eq!(Solution::inorder_traversal(root), vec![1, 3, 2]);
    }

    fn node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(n)))
    }
}
