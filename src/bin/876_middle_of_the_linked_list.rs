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

pub struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut head = head;
        let mut current = &mut head;
        let mut nodes = 0;
        while current.is_some() {
            nodes += 1;
            current = &mut current.as_mut().unwrap().next;
        }

        current = &mut head;
        for _i in 1..nodes {
            current = &mut current.as_mut().unwrap().next;
        }

        return current.take();
    }
}

fn main() {
    assert_eq!(None, Solution::middle_node(None));
    assert_eq!(Option::Some(Box::from(ListNode::new(1))),
               Solution::middle_node(Option::Some(Box::from(ListNode::new(1)))));
}