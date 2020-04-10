use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::BorrowMut;

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

pub struct Solution {}

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        if root.is_none() {
            return Vec::new();
        }
        let mut values: Vec<i32> = Vec::new();
        let mut output: Vec<String> = Vec::new();
        let mut root = root;

        traverse_recursively(&mut root, &mut values, &mut output);

        return output;

        pub fn traverse_recursively(current_node: &mut Option<Rc<RefCell<TreeNode>>>,
                                    values: &mut Vec<i32>, output: &mut Vec<String>) {
            match current_node {
                None => {
                    return;
                },
                Some(node_value) => {
                    let mut mut_node = node_value.try_borrow_mut().unwrap();
                    values.push(mut_node.val);

                    if mut_node.left.is_none() && mut_node.right.is_none() {
                        output.push(values.into_iter()
                            .map(|val| val.to_string())
                            .collect::<Vec<String>>()
                            .join("->"));
                    } else {
                        traverse_recursively(&mut mut_node.left,
                                             values.borrow_mut(), output.borrow_mut());
                        traverse_recursively(&mut mut_node.right,
                                             values.borrow_mut(), output.borrow_mut());
                    }
                    values.pop();
                }
            }
        }
    }
}

fn main() {
    assert_eq!(Vec::<String>::new(), Solution::binary_tree_paths(None));
    assert_eq!(vec!["1".to_string()], Solution::binary_tree_paths(Option::Some(Rc::from(RefCell::new(TreeNode::new(1))))))
}