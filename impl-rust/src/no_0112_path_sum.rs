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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let root = root.as_ref().unwrap().borrow();
        let root_val = root.val;
        // 不能判断 > sum，因为路径中可能有负数。
        // 这条链的和比 sum 大，返回 false。
        // if root_val > sum {
        //     return false;
        // }

        // 也不能这样判断，因为等于 sum 时如果不是叶子节点，不能排除没有满足的叶子节点。
        // 如果正好等于 sum，并且 root 是叶子节点，那么就返回 true。
        // if root_val == sum {
        //     return root.left.is_none() && root.right.is_none();
        // }
        if root.left.is_none() && root.right.is_none() {
            return root_val == sum;
        }
        let sum = sum - root_val;
        Self::has_path_sum(root.left.clone(), sum) || Self::has_path_sum(root.right.clone(), sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_path_sum1() {
        let root = TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 13,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        };

        assert!(Solution::has_path_sum(
            Some(Rc::new(RefCell::new(root))),
            22
        ));
    }

    #[test]
    fn test_has_path_sum2() {
        // 空的和 0 不能比较。。。
        assert_eq!(Solution::has_path_sum(None, 0), false);
    }

    #[test]
    fn test_has_path_sum3() {
        // 必须是到叶子节点。
        let root = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        };
        assert_eq!(
            Solution::has_path_sum(Some(Rc::new(RefCell::new(root))), 1),
            false
        );
    }

    #[test]
    fn test_has_path_sum4() {
        // 必须是到叶子节点。
        let root = TreeNode {
            val: -2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(-3)))),
        };
        assert_eq!(
            Solution::has_path_sum(Some(Rc::new(RefCell::new(root))), -5),
            true
        );
    }

    #[test]
    fn test_has_path_sum5() {
        // 路径中间可能达到了 sum，但不是叶子节点。而有叶子节点是满足条件的。
        let root = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(-2)))),
                right: None,
            }))),
        };
        assert_eq!(
            Solution::has_path_sum(Some(Rc::new(RefCell::new(root))), -1),
            true
        );
    }
}
