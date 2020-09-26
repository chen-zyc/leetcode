use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }
        // 每个元素的下标，元素没有重复的，所以可以这样。
        let idx_map: HashMap<i32, usize> = inorder
            .iter()
            .enumerate()
            .map(|(idx, ele)| (*ele, idx))
            .collect();
        Self::helper(0, inorder.len() - 1, &mut postorder, &idx_map)
    }

    fn helper(
        inorder_left: usize,
        inorder_right: usize,
        postorder: &mut Vec<i32>,
        idx_map: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // 没有剩余的节点了
        if inorder_left > inorder_right {
            return None;
        }

        // 后序遍历的最后一个元素就是当前子树的根节点
        if let Some(root_value) = postorder.pop() {
            let mut root = TreeNode::new(root_value);
            // 根在中序遍历中的位置
            if let Some(inorder_root_idx) = idx_map.get(&root_value) {
                root.right = Self::helper(inorder_root_idx + 1, inorder_right, postorder, idx_map);
                if *inorder_root_idx > 0 {
                    root.left =
                        Self::helper(inorder_left, inorder_root_idx - 1, postorder, idx_map);
                }
            }
            return Some(Rc::new(RefCell::new(root)));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tree_node;

    #[test]
    fn test_build_tree() {
        let ans = Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]);
        let want = tree_node(TreeNode {
            val: 3,
            left: tree_node(TreeNode::new(9)),
            right: tree_node(TreeNode {
                val: 20,
                left: tree_node(TreeNode::new(15)),
                right: tree_node(TreeNode::new(7)),
            }),
        });
        assert_eq!(ans, want);
    }
}
