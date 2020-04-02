use std::borrow::{Borrow};
use std::collections::VecDeque;

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
    fn new_with_next(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode {
            val,
            next
        }
    }
}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut last_val: Option<i32> = None;
        let mut current_node = head.borrow();
        let mut values: VecDeque<i32> = VecDeque::new();
        loop {
            let current_val = current_node.as_ref().unwrap().val;
            if let Some(last) = last_val {
                if last != current_val {
                    values.push_front(current_val);
                    last_val = Option::Some(current_val);
                }
            }
            else if let None = last_val {
                values.push_front(current_val);
                last_val = Option::Some(current_val);
            }

            current_node = &current_node.as_ref().unwrap().next;
            if current_node.is_none() {
                break;
            }
        }

        let mut result_head: Option<Box<ListNode>> = None;

        while let Some(val) = values.pop_front() {
            result_head = Option::Some(Box::from(
                ListNode::new_with_next(val, result_head)));
        }

        return result_head;
    }
}

fn main() {
    assert_eq!(None, Solution::delete_duplicates(None));
    assert_eq!(1, Solution::delete_duplicates(
        Option::Some(Box::from(ListNode::new(1)))).unwrap().val);
    assert_eq!(2, Solution::delete_duplicates(
        Option::Some(Box::from(ListNode::new_with_next(1,
        Option::Some(Box::from(ListNode::new_with_next(1,
        Option::Some(Box::from(ListNode::new(2))))))))))
        .unwrap().next.unwrap().val);
}
