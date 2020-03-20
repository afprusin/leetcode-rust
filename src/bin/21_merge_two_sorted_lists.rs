use std::borrow::{Borrow};

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

    fn new_with_ref(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { val, next }
    }

    fn push(val: i32, previous: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return Option::Some(Box::from(ListNode::new_with_ref(val, previous)));
    }
}

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1_iter = l1.borrow();
        let mut l2_iter = l2.borrow();

        let mut value_stack: Vec<i32> = Vec::new();
        while l1_iter.is_some() || l2_iter.is_some() {
            if let (Some(l1_val), None) = (l1_iter, l2_iter) {
                value_stack.push(l1_val.val);
                l1_iter = l1_val.next.borrow();
            } else if let (None, Some(l2_val)) = (l1_iter, l2_iter) {
                value_stack.push(l2_val.val);
                l2_iter = l2_val.next.borrow();
            } else if let (Some(l1_val), Some(l2_val)) = (l1_iter, l2_iter) {
                if l1_val.val <= l2_val.val {
                    value_stack.push(l1_val.val);
                    l1_iter = l1_val.next.borrow();
                } else {
                    value_stack.push(l2_val.val);
                    l2_iter = l2_val.next.borrow();
                }
            } else { panic!("Something something unreachable") }
        }

        let mut head_node = None;
        loop {
            match value_stack.pop() {
                Some(next_val) => head_node = ListNode::push(next_val, head_node),
                None => break
            }
        }

        return head_node.clone();
    }
}

fn main() {
    let mut result = Solution::merge_two_lists(
        Option::Some(Box::new(ListNode::new(0))),
        Option::Some(Box::new(ListNode::new(1))),
    );
    loop {
        if result.is_some() {
            let current = result.unwrap();
            println!("{}", current.val);
            result = current.next;
        }
        else {
            break;
        }
    }
}
