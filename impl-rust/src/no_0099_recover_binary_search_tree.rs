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
    // Morris 中序遍历
    // 参考 https://gist.github.com/upsuper/dc55bf156adbd194f5f8084bb59120f1
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let (mut x, mut y) = (None, None);
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        let mut cur_node = root.clone();
        while let Some(cur) = cur_node {
            let cur_ref = cur.borrow();
            let cur_val = cur_ref.val;
            let mut predecessor = match &cur_ref.left {
                // 如果有左子节点，在下面找到它的前驱节点。
                Some(left) => left.clone(),
                None => {
                    // 没有左子节点，访问当前节点。
                    if let Some(p) = prev {
                        if p.borrow().val > cur_val {
                            y = Some(cur.clone());
                            if x.is_none() {
                                x = Some(p.clone());
                            }
                        }
                    }
                    prev = Some(cur.clone());
                    // 然后继续右子节点
                    cur_node = cur_ref.right.clone();
                    continue;
                }
            };

            // 当前节点的左子节点不为空，找到它的前驱节点。
            loop {
                let right = match &predecessor.borrow().right {
                    Some(r) if Rc::ptr_eq(r, &cur) => break,
                    Some(r) => r.clone(),
                    None => break,
                };
                predecessor = right;
            }
            let mut predecessor_mut = predecessor.borrow_mut();
            match predecessor_mut.right {
                None => {
                    // 前驱节点的右子节点为空，将当前节点当作它的右子节点。
                    predecessor_mut.right = Some(cur.clone());
                    // 继续当前节点的左子节点
                    cur_node = cur_ref.left.clone();
                }
                Some(_) => {
                    predecessor_mut.right = None;
                    // 必须删除可变借用，不然下面 p.borrow() 时就借不出来了。
                    drop(predecessor_mut);

                    if let Some(p) = prev {
                        if p.borrow().val > cur_val {
                            y = Some(cur.clone());
                            if x.is_none() {
                                x = Some(p.clone());
                            }
                        }
                    }
                    prev = Some(cur.clone());
                    // 前驱节点的右子节点为当前节点，则将它的右子节点置为空。
                    cur_node = cur_ref.right.clone();
                }
            }
        }

        if let (Some(x), Some(y)) = (x, y) {
            std::mem::swap(&mut x.borrow_mut().val, &mut y.borrow_mut().val);
        }
    }

    // 隐式中序遍历。在遍历的过程中查找两个交换的节点。
    // 抄的别人的，没有找到链接。
    pub fn recover_tree2(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let (mut first, mut second, mut prev) = (None, None, None);
        Self::dfs(root, &mut first, &mut second, &mut prev);

        if let (Some(f), Some(s)) = (first, second) {
            std::mem::swap(&mut f.borrow_mut().val, &mut s.borrow_mut().val);
        }
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        first: &mut Option<Rc<RefCell<TreeNode>>>,
        second: &mut Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(r) = root {
            Self::dfs(&r.borrow().left, first, second, prev);
            if let Some(p) = prev {
                if p.borrow().val > r.borrow().val {
                    *second = Some(r.clone());
                    if first.is_none() {
                        *first = Some(p.clone());
                    }
                }
            }
            *prev = Some(r.clone());
            Self::dfs(&r.borrow().right, first, second, prev);
        }
    }

    // 显示中序遍历
    pub fn recover_tree1(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut nodes = Vec::new();
        Self::inorder(root, &mut nodes);

        // 找到两个交换的节点
        let mut x = None;
        let mut y = None;

        for i in 0..(nodes.len() - 1) {
            // 如果有不符合递增顺序的
            if nodes[i].borrow().val > nodes[i + 1].borrow().val {
                y = Some(nodes[i + 1].clone());
                if x.is_none() {
                    x = Some(nodes[i].clone());
                } else {
                    // 已经找到两个了，最多就两个。
                    break;
                }
            }
        }

        if let (Some(x), Some(y)) = (x, y) {
            std::mem::swap(&mut x.borrow_mut().val, &mut y.borrow_mut().val);
        }
    }

    // 把树形结构按中序遍历的顺序返回
    fn inorder(root: &mut Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let root_cloned = root.clone().unwrap();
        let mut r = root.as_mut().unwrap().borrow_mut();
        Self::inorder(&mut r.left, nodes);
        nodes.push(root_cloned);
        Self::inorder(&mut r.right, nodes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recover_tree1() {
        let mut root = node(TreeNode {
            val: 1,
            left: node(TreeNode {
                val: 3,
                left: None,
                right: node(TreeNode::new(2)),
            }),
            right: None,
        });
        Solution::recover_tree(&mut root);

        let want = node(TreeNode {
            val: 3,
            left: node(TreeNode {
                val: 1,
                left: None,
                right: node(TreeNode::new(2)),
            }),
            right: None,
        });
        assert_eq!(want, root);
    }

    #[test]
    fn test_recover_tree2() {
        let mut root = node(TreeNode {
            val: 3,
            left: node(TreeNode::new(1)),
            right: node(TreeNode {
                val: 4,
                left: node(TreeNode::new(2)),
                right: None,
            }),
        });
        Solution::recover_tree(&mut root);

        let want = node(TreeNode {
            val: 2,
            left: node(TreeNode::new(1)),
            right: node(TreeNode {
                val: 4,
                left: node(TreeNode::new(3)),
                right: None,
            }),
        });
        assert_eq!(want, root);
    }

    fn node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(n)))
    }
}
