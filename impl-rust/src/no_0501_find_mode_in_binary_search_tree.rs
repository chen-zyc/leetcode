use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Answer {
    base: i32,
    count: u32,
    max_count: u32,
    v: Vec<i32>,
}

impl Answer {
    fn new() -> Self {
        Answer {
            base: 0,
            count: 0,
            max_count: 0,
            v: Vec::new(),
        }
    }
    fn update(&mut self, n: i32) {
        if self.base == n {
            self.count += 1;
        } else {
            self.base = n;
            self.count = 1;
        }

        if self.count > self.max_count {
            self.v.clear();
            self.v.push(n);
            self.max_count = self.count;
        } else if self.count == self.max_count {
            self.v.push(n);
        }
    }
}

struct Solution;
impl Solution {
    // 用 Rust 实现 Morris 遍历有点难。
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = Answer::new();
        Self::mid_order(&root, &mut ans);
        ans.v
    }

    fn mid_order(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Answer) {
        if let Some(r) = root {
            Self::mid_order(&r.borrow().left, ans);
            ans.update(r.borrow().val);
            Self::mid_order(&r.borrow().right, ans);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tree_node;

    #[test]
    fn test_find_mode() {
        let root = tree_node(TreeNode {
            val: 1,
            left: None,
            right: tree_node(TreeNode {
                val: 2,
                left: tree_node(TreeNode::new(2)),
                right: None,
            }),
        });
        assert_eq!(Solution::find_mode(root), vec![2]);
    }
}
