use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (with_cam, _, watch_by_son) = Self::dfs(&root);
        // 根节点没有父节点，所以要么放置相机，要么被儿子监控
        with_cam.min(watch_by_son)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        match root {
            Some(n) => {
                let (left_with_cam, left_watch_by_dad, left_watch_by_son) =
                    Self::dfs(&n.borrow().left);
                let (right_with_cam, right_watch_by_dad, right_watch_by_son) =
                    Self::dfs(&n.borrow().right);

                // 当前节点放了相机
                let with_cam = 1
                    + (left_watch_by_dad + right_watch_by_dad) // 左右都不放
                        .min(left_with_cam + right_watch_by_dad) // 左边放了，右边不放
                        .min(left_watch_by_dad + right_with_cam); // 右边放了，左边不放

                // 当前节点没有放相机，但被父节点监控到了
                let watch_by_dad = (left_with_cam + right_with_cam) // 左右都放
                    .min(left_with_cam + right_watch_by_son) // 左边放，右边只能让它的儿子监控它了
                    .min(left_watch_by_son + right_with_cam) // 右边放，左边只能让它的儿子监控它了
                    .min(left_watch_by_son + right_watch_by_son); // 两边都不放，都让它们的儿子来监控

                // 当前节点没有放相机，也没有被父节点监控到
                let watch_by_son = (left_with_cam + right_with_cam) // 两边都放
                    .min(left_with_cam + right_watch_by_son) // 左边放了，右边让儿子监控
                    .min(left_watch_by_son + right_with_cam); // 右边放了，左边让儿子监控

                (with_cam, watch_by_dad, watch_by_son)
            }
            // 如果当前节点是空的，那么就不能放置相机，如果被父节点监控到了，那就不需要放相机了。如果没有被父节点监控到，也不需要放相机。
            // 除以 2 是为了防止相加时溢出。
            None => (std::i32::MAX / 2, 0, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tree_node;

    #[test]
    fn test_min_camera_cover1() {
        let root = tree_node(TreeNode {
            val: 0,
            left: tree_node(TreeNode {
                val: 0,
                left: tree_node(TreeNode::new(0)),
                right: tree_node(TreeNode::new(0)),
            }),
            right: None,
        });
        assert_eq!(Solution::min_camera_cover(root), 1);
    }

    #[test]
    fn test_min_camera_cover2() {
        let root = tree_node(TreeNode {
            val: 0,
            left: tree_node(TreeNode {
                val: 0,
                left: tree_node(TreeNode {
                    val: 0,
                    left: tree_node(TreeNode {
                        val: 0,
                        left: None,
                        right: tree_node(TreeNode::new(0)),
                    }),
                    right: None,
                }),
                right: None,
            }),
            right: None,
        });
        assert_eq!(Solution::min_camera_cover(root), 2);
    }
}
