use std::rc::Rc;
use std::cell::RefCell;

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

struct Solution {}

impl Solution {
    pub fn is_valid_sequence(root: Option<Rc<RefCell<TreeNode>>>, arr: Vec<i32>) -> bool {
        return Solution::is_valid_recursively(&root, &arr, 0);
    }

    fn is_valid_recursively(node: &Option<Rc<RefCell<TreeNode>>>, values: &Vec<i32>, index: usize) -> bool {
        return match node.as_ref() {
            None => false,
            Some(node) => {
                let node_contents = node.borrow();
                if node_contents.val != values[index] {
                    return false;
                }
                if index == values.len() - 1 {
                    return node_contents.left.is_none() && node_contents.right.is_none();
                }
                Solution::is_valid_recursively(
                    &node_contents.left, &values, index + 1) ||
                    Solution::is_valid_recursively(
                        &node_contents.right, &values, index + 1)
            }
        }
    }
}

fn main() {

}