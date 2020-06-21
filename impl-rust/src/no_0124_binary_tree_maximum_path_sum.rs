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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = std::i32::MIN;
        Self::max_gain(root, &mut max);
        max
    }

    fn max_gain(node: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if node.is_none() {
            return 0;
        }

        let root = node.as_ref().unwrap().borrow();

        let left_node = root.left.clone();
        let left_gain = Self::max_gain(left_node, max).max(0);
        let right_node = root.right.clone();
        let right_gain = Self::max_gain(right_node, max).max(0);

        *max = (root.val + left_gain + right_gain).max(*max);

        root.val + left_gain.max(right_gain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_path_sum_example1() {
        let mut root = TreeNode::new(1);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let max = Solution::max_path_sum(Some(Rc::new(RefCell::new(root))));
        assert_eq!(max, 6);
    }

    #[test]
    fn test_max_path_sum_example2() {
        let mut root = TreeNode::new(-10);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        root.right
            .as_ref()
            .map(|n| n.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15)))));
        root.right
            .as_ref()
            .map(|n| n.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7)))));
        let max = Solution::max_path_sum(Some(Rc::new(RefCell::new(root))));
        assert_eq!(max, 42);
    }

    #[test]
    fn test_max_path_sum_example3() {
        let root = TreeNode::new(-3);
        let max = Solution::max_path_sum(Some(Rc::new(RefCell::new(root))));
        assert_eq!(max, -3); // 只能选这一个。
    }
}
