use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (t1, t2) {
            (Some(n1), Some(n2)) => {
                {
                    let mut n = n1.borrow_mut();
                    n.val += n2.borrow().val;
                    n.left = Self::merge_trees(n.left.take(), n2.borrow_mut().left.take());
                    n.right = Self::merge_trees(n.right.take(), n2.borrow_mut().right.take());
                }
                Some(n1)
            }
            (Some(n), None) | (None, Some(n)) => Some(n),
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tree_node;

    #[test]
    fn test_merge_trees() {
        let t1 = tree_node(TreeNode {
            val: 1,
            left: tree_node(TreeNode {
                val: 3,
                left: tree_node(TreeNode::new(5)),
                right: None,
            }),
            right: tree_node(TreeNode::new(2)),
        });

        let t2 = tree_node(TreeNode {
            val: 2,
            left: tree_node(TreeNode {
                val: 1,
                left: None,
                right: tree_node(TreeNode::new(4)),
            }),
            right: tree_node(TreeNode {
                val: 3,
                left: None,
                right: tree_node(TreeNode::new(7)),
            }),
        });

        let ans = Solution::merge_trees(t1, t2);
        let want = tree_node(TreeNode {
            val: 3,
            left: tree_node(TreeNode {
                val: 4,
                left: tree_node(TreeNode::new(5)),
                right: tree_node(TreeNode::new(4)),
            }),
            right: tree_node(TreeNode {
                val: 5,
                left: None,
                right: tree_node(TreeNode::new(7)),
            }),
        });
        assert_eq!(ans, want);
    }
}
