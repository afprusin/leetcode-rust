use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;

pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1_iter = l1.borrow();
        let mut l2_iter = l2.borrow();

        let mut head_node = None.borrow_mut();
        let mut current_node = None.borrow_mut();
        while !l1_iter.is_none() || !l2_iter.is_none() {
            let &mut next_val;
            if let (Some(l1_val), None) = (l1_iter, l2_iter) {
                next_val = l1_val.val;
                l1_iter = l1_val.next.borrow();
            } else if let (None, Some(l2_val)) = (l1_iter, l2_iter) {
                next_val = l2_val.val;
                l2_iter = l2_val.next.borrow();
            } else if let (Some(l1_val), Some(l2_val)) = (l1_iter, l2_iter) {
                if l1_val.val <= l2_val.val {
                    next_val = l2_val.val;
                    l2_iter = l2_val.next.borrow();
                } else {
                    next_val = l1_val.val;
                    l1_iter = l1_val.next.borrow();
                }
            }
            if current_node.is_none() {
                current_node = &mut Option::Some(Box::from(ListNode::new(next_val)));
                head_node = current_node;
            } else {
                let &mut next_node = &mut ListNode::new(next_val).borrow_mut();
                next_node.next = current_node.clone();
                current_node = Option::Some(Box::from(next_node));
            }
        }

        return head_node.clone();
    }
}

fn main() {
    println!(
        "{}",
        Solution::merge_two_lists(
            Option::Some(Box::new(ListNode::new(0))),
            Option::Some(Box::new(ListNode::new(1))),
        )
        .unwrap()
        .val
    );
}
