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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return Vec::new();
        }
        Self::helper(1, n)
    }
    fn helper(left: i32, right: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if left > right {
            // NOTE: 这里一定要传一个空的元素，不能返回空数组，不然可能有一颗树没有节点。
            return vec![None];
        }
        let mut res = Vec::new();
        for root in left..=right {
            let left_nodes = Self::helper(left, root - 1);
            let right_nodes = Self::helper(root + 1, right);
            for left_node in left_nodes.iter() {
                for right_node in right_nodes.iter() {
                    res.push(Some(Rc::new(RefCell::new(TreeNode {
                        val: root,
                        left: left_node.clone(),
                        right: right_node.clone(),
                    }))));
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_trees() {
        let res = Solution::generate_trees(3);
        let want = vec![
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: None,
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
                right: None,
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: None,
                }))),
                right: None,
            }))),
        ];

        assert_eq!(res, want);
    }
}
