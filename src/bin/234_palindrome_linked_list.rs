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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }
        let mut head = head;
        let mut current = &mut head;
        let mut characters: Vec<i32> = Vec::new();
        while current.is_some() {
            characters.push(current.as_mut().unwrap().val);
            current = &mut current.as_mut().unwrap().next;
        }

        let length = characters.len() - 1;
        for i in 0..characters.len() / 2 {
            if characters[i] != characters[length - i] {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    assert_eq!(true, Solution::is_palindrome(None));
    assert_eq!(true, Solution::is_palindrome(Option::Some(Box::from(ListNode::new(1)))));
}