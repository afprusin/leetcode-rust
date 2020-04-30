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

pub struct Solution {}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let branch_and_path_maximum = Solution::search_recursively(&root);
        return branch_and_path_maximum.unwrap().1;
    }

    // TODO: The optional logic here gets very messy.  There's probably a way to simplify it
    fn search_recursively(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<(i32, i32)> {
        match root {
            None => return None,
            Some(node) => {
                let borrowed = node.borrow();
                let left_branch_path_maxes =
                    Solution::search_recursively(&borrowed.left);
                let right_branch_and_path_maxes =
                    Solution::search_recursively(&borrowed.right);

                let mut max_path = borrowed.val;
                let mut max_branch = borrowed.val;
                if let (Some(left_maxes), Some(right_maxes)) =
                        (left_branch_path_maxes, right_branch_and_path_maxes) {
                    let max_sub_branch = i32::max(left_maxes.0, right_maxes.0);
                    if max_sub_branch > 0 {
                        max_branch += max_sub_branch;
                    }
                    if left_maxes.0 > 0 {
                        max_path += left_maxes.0;
                    }
                    if right_maxes.0 > 0 {
                        max_path += right_maxes.0;
                    }
                    let max_sub_path = i32::max(left_maxes.1, right_maxes.1);
                    max_path = i32::max(max_path, max_sub_path);
                } else if let Some(left_maxes) = left_branch_path_maxes {
                    if left_maxes.0 > 0 {
                        max_branch += left_maxes.0;
                        max_path += left_maxes.0;
                    }
                    max_path = i32::max(max_path, left_maxes.1);
                } else if let Some(right_maxes) = right_branch_and_path_maxes {
                    if right_maxes.0 > 0 {
                        max_branch += right_maxes.0;
                        max_path += right_maxes.0;
                    }
                    max_path = i32::max(max_path, right_maxes.1);
                }
                return Option::Some((max_branch, max_path));
            }
        }
    }
}

fn main() {
    assert_eq!(0, Solution::max_path_sum(None));
    assert_eq!(7, Solution::max_path_sum(Option::Some(Rc::from(RefCell::from(TreeNode::new(7))))));
}
