use std::cell::RefCell;
use std::rc::Rc;

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

pub fn tree2str(root: Node) -> String {
    fn traverse(node: Node, s: &mut String) {
        if let Some(node) = node {
            let node = node.borrow();
            *s += node.val.to_string().as_str();
            if node.left.is_some() || node.right.is_some() {
                *s += "(";
                traverse(node.left.clone(), s);
                *s += ")";

                if node.right.is_some() {
                    *s += "(";
                    traverse(node.right.clone(), s);
                    *s += ")";
                }
            }
        }
    }

    let mut s = String::new();
    traverse(root, &mut s);
    s
}
