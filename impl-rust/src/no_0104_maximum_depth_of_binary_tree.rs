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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 深度遍历
        if root.is_none() {
            return 0;
        }
        let r = root.as_ref().unwrap().borrow();
        // 和下面那种方式在内存占用上并没有太大的区别。
        Self::max_depth(r.left.clone()).max(Self::max_depth(r.right.clone())) + 1
    }

    pub fn max_depth1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 深度遍历
        if root.is_none() {
            return 0;
        }
        let r = root.as_ref().unwrap().borrow();
        let left_depth = Self::max_depth(r.left.clone());
        let right_depth = Self::max_depth(r.right.clone());
        left_depth.max(right_depth) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        assert_eq!(Solution::max_depth(root), 3);
    }
}
