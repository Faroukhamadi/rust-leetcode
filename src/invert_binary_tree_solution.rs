use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

type Node<T> = Option<Rc<RefCell<TreeNode<T>>>>;

impl<T> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn invert_tree<T>(root: Node<T>) -> Node<T> {
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
