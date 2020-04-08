use std::collections::VecDeque;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut head = head;
        let mut current = &mut head;
        let mut values: VecDeque<i32> = VecDeque::new();

        while let Some(current_node) = current {
            values.push_front(current_node.val);
            current = &mut current_node.next;
        }

        let mut result_head = Option::Some(Box::from(ListNode::new(values.pop_front().unwrap())));
        let mut result_current = &mut result_head;
        while let Some(value) = values.pop_front() {
            result_current.as_mut().unwrap().next = Option::Some(Box::from(ListNode::new(value)));
            result_current = &mut result_current.as_mut().unwrap().next;
        }

        return result_head;
    }
}

fn main() {
    assert_eq!(None, Solution::reverse_list(Option::None));
    assert_eq!(Option::Some(Box::from(ListNode::new(77))),
               Solution::reverse_list(Option::Some(Box::from(ListNode::new(77)))));
}