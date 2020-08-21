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
use std::collections::LinkedList;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::min_depth_bfs(root)
    }

    fn min_depth_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        // 不能用栈，因为要一层一层的访问，这样碰到的第一个叶子节点都最浅的。
        let mut queue = LinkedList::new();
        queue.push_back((root, 1));
        while let Some((Some(node), depth)) = queue.pop_front() {
            let node = node.borrow();
            let (left, right) = (node.left.clone(), node.right.clone());
            // 找到一个叶子节点，这个就是最靠近根的叶子节点了。
            if left.is_none() && right.is_none() {
                return depth;
            }
            if left.is_some() {
                queue.push_back((left, depth + 1));
            }
            if right.is_some() {
                queue.push_back((right, depth + 1));
            }
        }

        // 不应该会到这儿吧
        0
    }

    pub fn min_depth1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::min_depth_dfs(root.as_ref())
    }

    fn min_depth_dfs(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        // 不能这么简单的用两边的最小值，如果左子树有，而右子树没有，那么树深是0，但应该是左子树的深度。
        // if let Some(r) = root.as_ref() {
        //     return 1 + Self::min_depth_dfs(&r.borrow().left)
        //         .min(Self::min_depth_dfs(&r.borrow().right));
        // }
        if let Some(r) = root.as_ref() {
            return match (&r.borrow().left, &r.borrow().right) {
                (Some(l), Some(r)) => {
                    1 + Self::min_depth_dfs(Some(l)).min(Self::min_depth_dfs(Some(r)))
                }
                // 下面两句可以用一个来表达。
                // (Some(l), None) => 1 + Self::min_depth_dfs(Some(l)),
                // (None, Some(r)) => 1 + Self::min_depth_dfs(Some(r)),
                (Some(n), None) | (None, Some(n)) => 1 + Self::min_depth_dfs(Some(n)),
                (None, None) => 1,
            };
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_depth1() {
        let root = node(TreeNode {
            val: 3,
            left: node(TreeNode::new(9)),
            right: node(TreeNode {
                val: 20,
                left: node(TreeNode::new(15)),
                right: node(TreeNode::new(7)),
            }),
        });
        assert_eq!(Solution::min_depth(root), 2);
    }

    #[test]
    fn test_min_depth2() {
        let root = node(TreeNode {
            val: 1,
            left: node(TreeNode::new(2)),
            right: None,
        });
        assert_eq!(Solution::min_depth(root), 2);
    }

    fn node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(n)))
    }
}
