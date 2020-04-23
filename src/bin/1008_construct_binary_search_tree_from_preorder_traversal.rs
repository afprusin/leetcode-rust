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
            right: None,
        }
    }

    pub fn new_with_next(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>) -> Self {
        TreeNode {
            val,
            left,
            right,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        let mut values = preorder;
        values.reverse();

        return Solution::build_recursively(None,&mut values);
    }

    fn build_recursively(max: Option<i32>, values: &mut Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }

        let node_value = values[values.len() - 1];
        if let Some(max) = max {
            if node_value > max {
                return None;
            }
        }
        values.pop();

        return Option::Some(Rc::from(RefCell::new(TreeNode::new_with_next(
            node_value,
            Solution::build_recursively(Some(node_value), values),
            Solution::build_recursively(max, values)))));
    }
}

fn main() {
    assert_eq!(None, Solution::bst_from_preorder(vec![]));
    assert_eq!(Option::Some(Rc::from(RefCell::new(TreeNode::new(1)))), Solution::bst_from_preorder(vec![1]));
}