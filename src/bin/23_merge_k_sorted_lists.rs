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
    // TODO: This could be accomplished reassigning the node's pointers to 'next', but I'm not well
    //  enough versed in ownership and passing references to handle it smoothly
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut values: Vec<i32> = lists.iter()
            .flat_map(|node| Solution::collect_node_values(node))
            .collect();

        if values.is_empty() {
            return None;
        }
        values.sort();
        let mut current = ListNode::new(values.pop().unwrap());
        values.reverse();
        for value in values {
            let mut previous = ListNode::new(value);
            previous.next = Option::Some(Box::from(current));
            current = previous;
        }

        return Option::Some(Box::from(current));
    }

    fn collect_node_values(list: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut current = list;
        let mut result = Vec::new();

        while let Some(node) = current {
            result.push(node.val);
            current = &node.next;
        }

        return result;
    }
}

fn main() {
    assert_eq!(None, Solution::merge_k_lists(vec![]));
    assert_eq!(None, Solution::merge_k_lists(vec![None, None]));
    assert_eq!(Option::Some(Box::from(ListNode::new(1))),
               Solution::merge_k_lists(vec![None, Option::Some(Box::from(ListNode::new(1))), None]))
}

