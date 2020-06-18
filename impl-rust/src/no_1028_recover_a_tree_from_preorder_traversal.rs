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
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = LinkedList::<Rc<RefCell<TreeNode>>>::new();
        let mut append_node = |val, depth| {
            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            if depth == stack.len() {
                // 父节点就是栈中最后一个节点
                // 如果 stack 是空的，那么就不需要找父节点，直接在最后添加到栈中。
                stack.front_mut().map(|n| {
                    n.borrow_mut().left = Some(node.clone());
                });
            } else {
                // 向上找父节点
                // root 的 depth 是 0，此时栈长 1. depth == 1 时应该找 root，也就是栈长等于 1 的那个顶部节点。
                while stack.len() > depth {
                    stack.pop_front();
                }
                stack.front_mut().map(|n| {
                    n.borrow_mut().right = Some(node.clone());
                });
            }
            stack.push_front(node);
        };

        let mut val = None;
        let mut depth = 0;
        for c in s.chars() {
            if c == '-' {
                val = val.and_then(|v| {
                    append_node(v, depth);
                    depth = 0;
                    None
                });
                depth += 1;
                continue;
            }
            let v = val.get_or_insert(0);
            *v = *v * 10 + (c as u8 - '0' as u8) as i32;
        }
        // 最后一个节点
        val.map(|v| append_node(v, depth));

        // 返回栈中第一个节点
        stack.pop_back()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn bfs(tree: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut v = Vec::new();
        if tree.is_none() {
            return v;
        }
        let mut stack = LinkedList::<Option<Rc<RefCell<TreeNode>>>>::new();
        stack.push_back(tree);
        while !stack.is_empty() {
            let node = stack.pop_front().unwrap();
            v.push(node.as_ref().map(|n| n.borrow().val));

            if node.is_none() {
                continue;
            }

            let left = node
                .as_ref()
                .and_then(|n| n.borrow().left.as_ref().map(|left| left.clone()));

            let right = node
                .as_ref()
                .and_then(|n| n.borrow().right.as_ref().map(|right| right.clone()));
            if left.is_some() || right.is_some() {
                stack.push_back(left);
                stack.push_back(right);
            }
        }
        v
    }

    #[test]
    fn test_recover_from_preorder_example1() {
        let tree = Solution::recover_from_preorder("1-2--3--4-5--6--7".to_owned());
        let v = bfs(tree);
        assert_eq!(
            v,
            vec![
                Some(1),
                Some(2),
                Some(5),
                Some(3),
                Some(4),
                Some(6),
                Some(7)
            ]
        )
    }

    #[test]
    fn test_recover_from_preorder_example2() {
        let tree = Solution::recover_from_preorder("1-2--3---4-5--6---7".to_owned());
        let v = bfs(tree);
        assert_eq!(
            v,
            vec![
                Some(1),
                Some(2),
                Some(5),
                Some(3),
                None,
                Some(6),
                None,
                Some(4),
                None,
                Some(7),
                None,
            ]
        )
    }

    #[test]
    fn test_recover_from_preorder_example3() {
        let tree = Solution::recover_from_preorder("1-401--349---90--88".to_owned());
        let v = bfs(tree);
        assert_eq!(
            v,
            vec![
                Some(1),
                Some(401),
                None,
                Some(349),
                Some(88),
                Some(90),
                None,
            ]
        )
    }
}
