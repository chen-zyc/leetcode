use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn preorder_traversal1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        Self::dfs(&root, &mut ans);
        ans
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let root = root.as_ref().unwrap().borrow();
        ans.push(root.val);
        Self::dfs(&root.left, ans);
        Self::dfs(&root.right, ans);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tree_node;

    #[test]
    fn test_preorder_traversal() {
        let root = tree_node(TreeNode {
            val: 1,
            left: None,
            right: tree_node(TreeNode {
                val: 2,
                left: tree_node(TreeNode::new(3)),
                right: None,
            }),
        });
        let ans = Solution::preorder_traversal(root);
        assert_eq!(ans, vec![1, 2, 3]);
    }
}
