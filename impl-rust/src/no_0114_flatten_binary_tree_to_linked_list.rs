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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut curr = root.as_ref().map(|n| n.clone());
        while let Some(curr_node) = curr {
            let mut curr_node = curr_node.borrow_mut();
            if let Some(next_node) = curr_node.left.take() {
                // 寻找前驱节点
                // let mut predecessor = next_node.borrow_mut();
                // while predecessor.right.is_some() {
                //     // ERROR! cannot assign to `predecessor` because it is borrowed
                //     predecessor = predecessor.right.as_mut().unwrap().borrow_mut();
                // }
                let mut predecessor = next_node.clone();
                let mut predecessor_right = predecessor.borrow().right.clone();
                while let Some(node) = predecessor_right {
                    predecessor_right = node.borrow().right.clone();
                    predecessor = node;
                }
                // 右子树当作前驱节点的右子树
                predecessor.borrow_mut().right = curr_node.right.take();
                // 当前节点的左树当作右树
                curr_node.right = Some(next_node);
            }
            // 继续遍历右子节点
            curr = curr_node.right.clone();
        }
    }

    pub fn flatten_failed(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut mut_root = root.as_mut().unwrap().borrow_mut();
        let mut left = mut_root.left.take();
        Self::flatten_failed(&mut left);
        let mut right = mut_root.right.take();
        Self::flatten_failed(&mut right);

        // while let Some(node) = left.take() {
        //     left = node.borrow_mut().right.take();
        //     mut_root.right = Some(node);
        //     mut_root = mut_root.right.as_mut().unwrap().borrow_mut();
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten() {
        let mut root = build_one_node(TreeNode {
            val: 1,
            left: build_one_node(TreeNode {
                val: 2,
                left: build_one_node(TreeNode::new(3)),
                right: build_one_node(TreeNode::new(4)),
            }),
            right: build_one_node(TreeNode {
                val: 5,
                left: None,
                right: build_one_node(TreeNode::new(6)),
            }),
        });
        Solution::flatten(&mut root);

        let want = build_one_node(TreeNode {
            val: 1,
            left: None,
            right: build_one_node(TreeNode {
                val: 2,
                left: None,
                right: build_one_node(TreeNode {
                    val: 3,
                    left: None,
                    right: build_one_node(TreeNode {
                        val: 4,
                        left: None,
                        right: build_one_node(TreeNode {
                            val: 5,
                            left: None,
                            right: build_one_node(TreeNode::new(6)),
                        }),
                    }),
                }),
            }),
        });
        assert_eq!(root, want);
    }

    fn build_one_node(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(node)))
    }
}
