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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::tree_height(&root).is_some()
    }

    // 这里也可以不用 Option，只使用 i32，并规定 -1 表示不平衡。
    fn tree_height(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if root.is_none() {
            return Some(0);
        }
        let r = root.as_ref().unwrap().borrow();
        let left = Self::tree_height(&r.left);
        if left.is_none() {
            return None;
        }
        let right = Self::tree_height(&r.right);
        if right.is_none() {
            return None;
        }

        let (l, r) = (left.unwrap(), right.unwrap());
        if (l - r).abs() > 1 {
            return None;
        }

        Some(1 + l.max(r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_balanced1() {
        let root = node(TreeNode {
            val: 3,
            left: node(TreeNode::new(9)),
            right: node(TreeNode {
                val: 20,
                left: node(TreeNode::new(15)),
                right: node(TreeNode::new(7)),
            }),
        });
        assert!(Solution::is_balanced(root));
    }

    #[test]
    fn test_is_balanced2() {
        let root = node(TreeNode {
            val: 1,
            left: node(TreeNode {
                val: 2,
                left: node(TreeNode {
                    val: 3,
                    left: node(TreeNode::new(4)),
                    right: node(TreeNode::new(4)),
                }),
                right: node(TreeNode::new(3)),
            }),
            right: node(TreeNode::new(2)),
        });
        assert!(!Solution::is_balanced(root));
    }

    #[test]
    fn test_is_balanced3() {
        let root = node(TreeNode {
            val: 1,
            left: node(TreeNode {
                val: 2,
                left: node(TreeNode {
                    val: 3,
                    left: node(TreeNode::new(4)),
                    right: None,
                }),
                right: None,
            }),
            right: node(TreeNode {
                val: 2,
                left: None,
                right: node(TreeNode {
                    val: 3,
                    left: None,
                    right: node(TreeNode::new(4)),
                }),
            }),
        });
        assert!(!Solution::is_balanced(root));
    }

    fn node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(n)))
    }
}
