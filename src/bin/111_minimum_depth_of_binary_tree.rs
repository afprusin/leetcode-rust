use std::rc::Rc;
use std::cell::{RefCell};
use std::borrow::{Borrow, BorrowMut};
use std::collections::VecDeque;
use std::ops::Deref;
use std::thread::current;
use std::convert::TryInto;

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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut to_process: VecDeque<(i32, &Rc<RefCell<TreeNode>>)> = VecDeque::new();
        if let Some(root_value) = root {
            let cloned = root_value.clone();
            to_process.push_front((1, &cloned));
        }

        while let Some(current_pair) = to_process.pop_front() {
            let current_node = current_pair.1.try_borrow().unwrap().deref();

            if current_node.right.is_none() && current_node.left.is_none() {
                return current_pair.0;
            }
            if let Some(left) = &current_node.left {
                to_process.push_back((current_pair.0 + 1, left))
            }
            if let Some(right) = &current_node.right {
                to_process.push_back((current_pair.0 + 1, right))
            }
        }

        return -1;
    }
}

fn main() {
    assert_eq!(0, Solution::min_depth(Option::None));
    assert_eq!(1, Solution::min_depth(Option::Some(Rc::from(RefCell::from(TreeNode::new(11))))));
}
