use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        Self::dfs(&root, &mut ans);
        ans
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if let Some(r) = root {
            Self::dfs(&r.borrow().left, ans);
            Self::dfs(&r.borrow().right, ans);
            ans.push(r.borrow().val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tree_node;

    #[test]
    fn test_postorder_traversal() {
        let root = tree_node(TreeNode {
            val: 1,
            left: None,
            right: tree_node(TreeNode {
                val: 2,
                left: tree_node(TreeNode::new(3)),
                right: None,
            }),
        });
        assert_eq!(Solution::postorder_traversal(root), vec![3, 2, 1]);
    }
}
