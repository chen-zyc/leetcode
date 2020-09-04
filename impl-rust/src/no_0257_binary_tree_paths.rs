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
    // 这个不使用 Vec<i32> 来传递结果，而是直接在传递过程中生成字符串。
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut ans = Vec::new();
        if root.is_none() {
            return ans;
        }
        Self::dfs2(root.as_ref().unwrap(), String::new(), &mut ans);
        ans
    }

    fn dfs2(root: &Rc<RefCell<TreeNode>>, mut path: String, ans: &mut Vec<String>) {
        let root = root.borrow();
        path.push_str(&root.val.to_string());

        match (&root.left, &root.right) {
            (None, None) => {
                ans.push(path);
            }
            (Some(n), None) | (None, Some(n)) => {
                path.push_str("->");
                // 只有一个节点，把 path 的所有权转移给它。
                Self::dfs2(n, path, ans);
            }
            (Some(l), Some(r)) => {
                path.push_str("->");
                // 两个节点的话，把 path 的所有权转移给右子节点。
                Self::dfs2(l, path.clone(), ans);
                Self::dfs2(r, path, ans);
            }
        }
    }

    pub fn binary_tree_paths1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut ans = Vec::new();
        if root.is_none() {
            return ans;
        }
        let mut tmp = Vec::new();
        Self::dfs(root.as_ref().unwrap(), &mut tmp, &mut ans);
        ans
    }

    fn dfs(root: &Rc<RefCell<TreeNode>>, tmp: &mut Vec<i32>, ans: &mut Vec<String>) {
        let root = root.borrow();
        tmp.push(root.val);
        match (&root.left, &root.right) {
            (None, None) => {
                Self::generate_list(tmp, ans);
            }
            (Some(n), None) | (None, Some(n)) => {
                Self::dfs(n, tmp, ans);
            }
            (Some(l), Some(r)) => {
                Self::dfs(l, tmp, ans);
                Self::dfs(r, tmp, ans);
            }
        }
        tmp.pop();
    }

    fn generate_list(tmp: &Vec<i32>, ans: &mut Vec<String>) {
        let mut s = String::new();
        for (i, v) in tmp.iter().enumerate() {
            if i != 0 {
                s.push_str("->");
            }
            s.push_str(&v.to_string());
        }
        ans.push(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree_paths() {
        let root = node(TreeNode {
            val: 1,
            left: node(TreeNode {
                val: 2,
                left: None,
                right: node(TreeNode::new(5)),
            }),
            right: node(TreeNode::new(3)),
        });
        let want = vec!["1->2->5".to_owned(), "1->3".to_owned()];
        assert_eq!(Solution::binary_tree_paths(root), want);
    }

    fn node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(n)))
    }
}
