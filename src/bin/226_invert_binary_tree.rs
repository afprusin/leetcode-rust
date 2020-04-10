use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

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

pub struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let mut root = root;
        invert_recursive(&mut root);

        return root;

        fn invert_recursive(node: &mut Option<Rc<RefCell<TreeNode>>>) {
            match node {
                None => return,
                Some(node_val) => {
                    let mut node_val = node_val.try_borrow_mut().unwrap();
                    let temp_right = &mut mem::replace(&mut node_val.right, None);
                    let temp_left = &mut mem::replace(&mut node_val.left, None);

                    invert_recursive(temp_right);
                    invert_recursive(temp_left);

                    mem::replace(&mut node_val.right, temp_left.take());
                    mem::replace(&mut node_val.left, temp_right.take());
                }
            }
        }
    }
}

fn main() {

}