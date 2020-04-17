use std::borrow::Borrow;

pub struct Solution {}

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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k <= 1 {
            return head;
        }
        let mut head = head;
        let mut front = &head.as_ref().unwrap().next;
        let mut rear = front;



        fn reverse_nodes(mut front : &Option<Box<ListNode>>, mut rear: &Option<Box<ListNode>>, nodes: i32) {
            loop {
                for i in 1..k {
                    front = &front.as_ref().unwrap().next;
                    if front.is_none() {
                        return;
                    }
                }
            }

        }

    }
}

fn main() {
    // assert_eq!(None, Solution::merge_k_lists(vec![]));
    // assert_eq!(None, Solution::merge_k_lists(vec![None, None]));
    // assert_eq!(Option::Some(Box::from(ListNode::new(1))),
    //            Solution::merge_k_lists(vec![None, Option::Some(Box::from(ListNode::new(1))), None]))
}

