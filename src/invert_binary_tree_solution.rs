use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type Node = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn invert_tree(root: Node) -> Node {
    if let Some(node) = root.clone() {
        let mut node = node.borrow_mut();
        let (l, r) = (
            invert_tree(node.left.clone()),
            invert_tree(node.right.clone()),
        );
        node.left = r;
        node.right = l;
    }
    root
}
