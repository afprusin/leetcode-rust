use std::rc::Rc;
use std::cell::{RefCell};
use std::borrow::Borrow;

pub struct Solution {}

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
      right: None
    }
  }
}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recursive_max_depth(node: Option<&Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
            return match node {
                None => depth,
                Some(node_val) => {
                    let deref_node = (**node_val).borrow();
                    let left_depth = recursive_max_depth(deref_node.left.as_ref(), depth + 1);
                    let right_depth = recursive_max_depth(deref_node.right.as_ref(), depth + 1);
                    if left_depth > right_depth {
                        left_depth
                    } else {
                        right_depth
                    }
                }
            };
        }
        return recursive_max_depth(root.borrow().as_ref(), 0);
    }
}

fn main() {
    assert_eq!(0, Solution::max_depth(Option::None));
    assert_eq!(1, Solution::max_depth(Option::Some(Rc::from(RefCell::from(TreeNode::new(1))))));
}
