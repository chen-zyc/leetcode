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
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::to_bst(&nums, 0, nums.len())
    }

    fn to_bst(nums: &Vec<i32>, left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if left >= right {
            return None;
        }
        let mid = (left + right) / 2; // 中间点靠右
        let mut root = TreeNode::new(nums[mid]);

        // 其实可以不用计算数量的，因为前面有 left >= right 的判断了
        let left_nodes_count = mid - left; // 不包括 mid
        let right_nodes_count = right - 1 - mid; // 不包括 mid
        if left_nodes_count > 0 {
            root.left = Self::to_bst(nums, left, mid);
        }
        if right_nodes_count > 0 {
            root.right = Self::to_bst(nums, mid + 1, right);
        }

        println!(
            "root = {}, pos = ({}, {}, {}), l = {}, r = {}",
            nums[mid], left, mid, right, left_nodes_count, right_nodes_count
        );
        Some(Rc::new(RefCell::new(root)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_array_to_bst() {
        let tree = Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]);
        let want = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -10,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        assert_eq!(tree, want);
    }
}
