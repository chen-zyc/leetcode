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
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        if let Some(r) = root {
            let left = r.borrow_mut().left.take();
            ans += left.as_ref().map_or(0, |n| {
                let n = n.borrow();
                if n.left.is_none() && n.right.is_none() {
                    n.val
                } else {
                    0
                }
            });
            ans += Self::sum_of_left_leaves(left);
            ans += Self::sum_of_left_leaves(r.borrow_mut().right.take());
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_left_leaves() {
        let root = node(TreeNode {
            val: 3,
            left: node(TreeNode::new(9)),
            right: node(TreeNode {
                val: 20,
                left: node(TreeNode::new(15)),
                right: node(TreeNode::new(7)),
            }),
        });
        assert_eq!(Solution::sum_of_left_leaves(root), 24);
    }

    fn node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(n)))
    }
}
