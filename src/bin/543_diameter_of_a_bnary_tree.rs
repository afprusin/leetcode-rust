use std::rc::Rc;
use std::cell::RefCell;

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

struct Solution {}

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return recursively_find_depth_and_best(&root, 0).1;

        fn recursively_find_depth_and_best(node: &Option<Rc<RefCell<TreeNode>>>, depth: i32) -> (i32, i32) {
            match node {
                None => {
                    return (depth, 0);
                },
                Some(node_val) => {
                    let node_val = node_val.try_borrow().unwrap();
                    let left_depth = recursively_find_depth_and_best(&node_val.left, depth);
                    let right_depth = recursively_find_depth_and_best(&node_val.right, depth);

                    let child_best = i32::max(left_depth.1, right_depth.1);
                    let best = i32::max(child_best, left_depth.0 + right_depth.0);
                    let max_depth = i32::max(left_depth.0, right_depth.0);

                    return (max_depth + 1, best);
                }
            }
        }
    }
}

fn main() {

}