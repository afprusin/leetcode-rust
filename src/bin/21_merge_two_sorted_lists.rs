use std::borrow::{Borrow, BorrowMut};

pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {

    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1_iter = l1.borrow();
        let mut l2_iter = l2.borrow();

        let &mut head_node = None.borrow_mut();
        let &mut current_node = None.borrow_mut();
        while !l1_iter.is_none() || !l2_iter.is_none() {
            let next_node = match (l1_iter, l2_iter) {
                (None, None) => None.borrow_mut(),
                (Some(l1_iter), None) =>
                    Option::Some(Box::new(ListNode::new(l1_iter.val))).borrow_mut(),
                (None, Some(l2_iter)) =>
                    Option::Some(Box::new(ListNode::new(l2_iter.val))).borrow_mut(),
                (Some(l1_iter), Some(l2_iter)) =>
                    if l1_iter.val <= l2_iter.val {
                        Option::Some(Box::new(ListNode::new(l1_iter.val)));
                    } else {
                        Option::Some(Box::new(ListNode::new(l1_iter.val)));
                    }
            };
            match current_node {
                None => {
                    &head_node = current_node.borrow();
                    &current_node = next_node.borrow();
                }
                Some(&boxed_val) => {
                    current_node.next = next_node;
                    &current_node = next_node.borrow();
                }
            }
        }

        return head_node;
    }

}

fn main() {
    println!("{}", Solution::merge_two_lists(
        Option::Some(Box::new(ListNode::new(0))),
        Option::Some(Box::new(ListNode::new(1))),
    ).unwrap().val);
}

