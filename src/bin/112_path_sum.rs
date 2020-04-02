use std::rc::Rc;
use std::cell::{RefCell};

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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        return depth_first_recursive(root.as_ref(), 0, sum);

        fn depth_first_recursive(node: Option<&Rc<RefCell<TreeNode>>>, total: i32, sum: i32) -> bool {
            return match node {
                None => { false },
                Some(node_some) => {
                    let node_ref = node_some.try_borrow().unwrap();
                    let new_total = total + node_ref.val;
                    if node_ref.left.is_none() && node_ref.right.is_none() {
                        new_total == sum
                    }
                    else {
                        depth_first_recursive(node_ref.left.as_ref(), new_total, sum) ||
                            depth_first_recursive(node_ref.right.as_ref(), new_total, sum)
                    }
                }
            };
        }
    }
}

fn main() {
    assert_eq!(false, Solution::has_path_sum(Option::None, 0));
}
