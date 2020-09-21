use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn convert_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&mut root, 0);
        root
    }

    fn helper(root: &mut Option<Rc<RefCell<TreeNode>>>, parent_val: i32) -> i32 {
        if let Some(r) = root {
            // parent 比当前节点大，也比右子树上的节点都大。
            let right_val = Self::helper(&mut r.borrow_mut().right, parent_val);

            // right_val 中已经包含了 parent_val，所以这里不需要再加上 parent_val 了。
            let val = r.borrow().val + right_val;
            r.borrow_mut().val = val;

            // val 中包含了比左子树大的所有节点的值（parent, right, root）
            return Self::helper(&mut r.borrow_mut().left, val);
        }

        // 如果一个节点的右子树为空，那么 parent_val 需要加到该节点上，所以这里最少要返回 parent_val。
        parent_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tree_node;

    #[test]
    fn test_convert_bst1() {
        let root = tree_node(TreeNode {
            val: 5,
            left: tree_node(TreeNode::new(2)),
            right: tree_node(TreeNode::new(13)),
        });
        let want = tree_node(TreeNode {
            val: 18,
            left: tree_node(TreeNode::new(20)),
            right: tree_node(TreeNode::new(13)),
        });
        assert_eq!(Solution::convert_bst(root), want);
    }

    #[test]
    fn test_convert_bst2() {
        let root = tree_node(TreeNode {
            val: 2,
            left: tree_node(TreeNode {
                val: 0,
                left: tree_node(TreeNode::new(-4)),
                right: tree_node(TreeNode::new(1)),
            }),
            right: tree_node(TreeNode::new(3)),
        });
        let want = tree_node(TreeNode {
            val: 5,
            left: tree_node(TreeNode {
                val: 6,
                left: tree_node(TreeNode::new(2)),
                right: tree_node(TreeNode::new(6)),
            }),
            right: tree_node(TreeNode::new(3)),
        });
        assert_eq!(Solution::convert_bst(root), want);
    }

    #[test]
    fn test_convert_bst3() {
        let root = tree_node(TreeNode {
            val: 1,
            left: tree_node(TreeNode {
                val: 0,
                left: tree_node(TreeNode::new(-2)),
                right: None,
            }),
            right: tree_node(TreeNode {
                val: 4,
                left: tree_node(TreeNode::new(3)),
                right: None,
            }),
        });
        let want = tree_node(TreeNode {
            val: 8,
            left: tree_node(TreeNode {
                val: 8,
                left: tree_node(TreeNode::new(6)),
                right: None,
            }),
            right: tree_node(TreeNode {
                val: 4,
                left: tree_node(TreeNode::new(7)),
                right: None,
            }),
        });
        assert_eq!(Solution::convert_bst(root), want);
    }
}
