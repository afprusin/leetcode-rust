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
    // Cribbed from user 'ashtower' - this example perfectly explains the use of references
    //  within immutable parameters
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut result= head;
        let mut current = &mut result;

        while current.is_some() {
            if current.as_ref().unwrap().val == val {
                *current = current.take().unwrap().next;
            } else {
                current = &mut current.as_mut().unwrap().next;
            }
        }
        return result;
    }
}

fn main() {
    assert_eq!(Option::None, Solution::remove_elements(Option::None, 1));
    assert_eq!(Option::None, Solution::remove_elements(Option::Some(Box::from(ListNode::new(1))), 1));
    assert_eq!(Option::Some(Box::from(ListNode::new(2))), Solution::remove_elements(Option::Some(Box::from(ListNode::new(2))), 1));
}