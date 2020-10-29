use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dfs(&root, 0, &mut ans);
        ans
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, mut sum: i32, ans: &mut i32) {
        if let Some(r) = root {
            sum = sum * 10 + r.borrow().val;
            if r.borrow().left.is_none() && r.borrow().right.is_none() {
                *ans += sum;
                return;
            }
            Self::dfs(&r.borrow().left, sum, ans);
            Self::dfs(&r.borrow().right, sum, ans);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tree_node;

    #[test]
    fn test_sum_numbers() {
        let root = tree_node(TreeNode {
            val: 1,
            left: tree_node(TreeNode::new(2)),
            right: tree_node(TreeNode::new(3)),
        });
        assert_eq!(Solution::sum_numbers(root), 25);

        let root = tree_node(TreeNode {
            val: 4,
            left: tree_node(TreeNode {
                val: 9,
                left: tree_node(TreeNode::new(5)),
                right: tree_node(TreeNode::new(1)),
            }),
            right: tree_node(TreeNode::new(0)),
        });
        assert_eq!(Solution::sum_numbers(root), 1026);
    }
}
