use std::rc::Rc;
use std::cell::RefCell;

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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn nodes_are_symmetric(left: Option<&Rc<RefCell<TreeNode>>>, right: Option<&Rc<RefCell<TreeNode>>>) -> bool {
            return match (left, right) {
                (None, None) => true,
                (Some(_), None) | (None, Some(_)) => false,
                (Some(left_val), Some(right_val)) => {
                    let l = left_val.borrow();
                    let r = right_val.borrow();
                    if l.val != r.val {
                        false
                    } else {
                        nodes_are_symmetric(l.left.as_ref(), r.right.as_ref()) &&
                            nodes_are_symmetric(l.right.as_ref(), r.left.as_ref())
                    }
                }
            }
        }

        return match root {
            None => true,
            Some(node) => {
                nodes_are_symmetric(node.borrow().left.as_ref(), node.borrow().right.as_ref())
            }
        };
    }
}

fn main() {
    assert_eq!(true, Solution::is_symmetric(Option::Some(Rc::from(RefCell::from(TreeNode::new(1))))));
}
