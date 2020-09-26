use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        Self::helper(&root, sum, &mut vec![], &mut ans);
        ans
    }

    fn helper(
        root: &Option<Rc<RefCell<TreeNode>>>,
        mut sum: i32,
        tmp: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if let Some(r) = root {
            let r = r.borrow();
            sum -= r.val;
            tmp.push(r.val);
            if sum == 0 && r.left.is_none() && r.right.is_none() {
                ans.push(tmp.clone());
            }
            if r.left.is_some() {
                Self::helper(&r.left, sum, tmp, ans);
            }
            if r.right.is_some() {
                Self::helper(&r.right, sum, tmp, ans);
            }
            tmp.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tree_node;

    #[test]
    fn test_path_sum() {
        let root = tree_node(TreeNode {
            val: 5,
            left: tree_node(TreeNode {
                val: 4,
                left: tree_node(TreeNode {
                    val: 11,
                    left: tree_node(TreeNode::new(7)),
                    right: tree_node(TreeNode::new(2)),
                }),
                right: None,
            }),
            right: tree_node(TreeNode {
                val: 8,
                left: tree_node(TreeNode::new(13)),
                right: tree_node(TreeNode {
                    val: 4,
                    left: tree_node(TreeNode::new(5)),
                    right: tree_node(TreeNode::new(1)),
                }),
            }),
        });
        let ans = Solution::path_sum(root, 22);
        let want = vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]];
        assert_eq!(ans, want);
    }

    #[test]
    fn test_path_sum2() {
        let root = tree_node(TreeNode {
            val: -2,
            left: None,
            right: tree_node(TreeNode::new(-3)),
        });
        let ans = Solution::path_sum(root, -5);
        let want = vec![vec![-2, -3]];
        assert_eq!(ans, want);
    }
}
