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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let res = Self::rob_helper(root);
        res.0.max(res.1)
    }

    // .0: 没有选中间节点时的最大值。
    // .1: 选了中间节点时的最大值。
    fn rob_helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if root.is_none() {
            return (0, 0);
        }
        let r = root.as_ref().unwrap().borrow();
        let left_res = Self::rob_helper(r.left.clone());
        let right_res = Self::rob_helper(r.right.clone());
        (
            // 没有选中间节点，左右子节点可以选(这里错了)
            // left_res.1 + right_res.1,
            // 没有选中间节点时，左右子节点可以选，也可以不选
            left_res.0.max(left_res.1) + right_res.0.max(right_res.1),
            // 选了中间节点，那么左右子节点就不能选了
            r.val + left_res.0 + right_res.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob1() {
        let root = node(TreeNode {
            val: 3,
            left: node(TreeNode {
                val: 2,
                left: None,
                right: node(TreeNode::new(3)),
            }),
            right: node(TreeNode {
                val: 3,
                left: None,
                right: node(TreeNode::new(1)),
            }),
        });
        assert_eq!(Solution::rob(root), 7);
    }

    #[test]
    fn test_rob2() {
        let root = node(TreeNode {
            val: 3,
            left: node(TreeNode {
                val: 4,
                left: node(TreeNode::new(1)),
                right: node(TreeNode::new(3)),
            }),
            right: node(TreeNode {
                val: 5,
                left: None,
                right: node(TreeNode::new(1)),
            }),
        });
        assert_eq!(Solution::rob(root), 9);
    }

    #[test]
    fn test_rob3() {
        let root = node(TreeNode {
            val: 4,
            left: node(TreeNode {
                val: 1,
                left: node(TreeNode {
                    val: 2,
                    left: node(TreeNode::new(3)),
                    right: None,
                }),
                right: None,
            }),
            right: None,
        });
        assert_eq!(Solution::rob(root), 7); // 选了开头的4和末尾的3.
    }

    fn node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(n)))
    }
}
