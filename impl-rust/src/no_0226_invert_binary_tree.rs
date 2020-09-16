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
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&mut root);
        root
    }

    fn helper(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut root = root.as_mut().unwrap().borrow_mut();
        let left = root.left.take();
        root.left = root.right.take();
        root.right = left;

        Self::helper(&mut root.left);
        Self::helper(&mut root.right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_tree() {
        let root = node(TreeNode {
            val: 4,
            left: node(TreeNode {
                val: 2,
                left: node(TreeNode::new(1)),
                right: node(TreeNode::new(3)),
            }),
            right: node(TreeNode {
                val: 7,
                left: node(TreeNode::new(6)),
                right: node(TreeNode::new(9)),
            }),
        });
        let want = node(TreeNode {
            val: 4,
            left: node(TreeNode {
                val: 7,
                left: node(TreeNode::new(9)),
                right: node(TreeNode::new(6)),
            }),
            right: node(TreeNode {
                val: 2,
                left: node(TreeNode::new(3)),
                right: node(TreeNode::new(1)),
            }),
        });
        let ans = Solution::invert_tree(root);
        assert_eq!(ans, want);
    }

    fn node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(n)))
    }
}
