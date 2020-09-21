use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

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

pub fn tree_node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(n)))
}

pub fn sort_2d_vec(v: &mut Vec<Vec<i32>>) {
    v.sort_by(|a, b| match a.len().cmp(&b.len()) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            for i in 0..a.len() {
                if a[i] != b[i] {
                    return a[i].cmp(&b[i]);
                }
            }
            Ordering::Equal
        }
    });
}
