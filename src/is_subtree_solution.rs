use std::{cell::RefCell, rc::Rc};

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

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn is_subtree(root: Node, sub_root: Node) -> bool {
    if root == sub_root {
        return true;
    }
    if let Some(node) = root {
        let node = node.borrow();

        is_subtree(node.left.clone(), sub_root.clone())
            || is_subtree(node.right.clone(), sub_root.clone())
    } else {
        false
    }
}
