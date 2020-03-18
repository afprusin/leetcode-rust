use test::NamePadding::PadNone;

pub struct Solution {}

//Definition for singly-linked list.
//#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode {
            next,
            val
        }
    }
}

pub struct LinkedList {
    pub head: Option<Box<ListNode>>,
    pub current: Option<Box<ListNode>>
}

impl Solution {

    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1.is_none() { return l2; }
        else if l2.is_none() { return none };

        let mut l1_iter = l1;
        let mut l2_iter = l2;

        let mut values: Vec<i32> = Vec::new();
        while !l1_iter.is_none() || !l2_iter.is_none() {
            if l1_iter.is_none() {
                values.push(l2_iter.unwrap().val);
            }
            else if l2_iter.is_none() {
                values.push(l1_iter.unwrap().val);
            }
            else if l1_iter.unwrap().val <= l2_iter.unwrap().val {
                values.push(l1_iter.unwrap().val);
            }
            else {
                values.push(l2_iter.unwrap().val);
            }
        }

        let mut head_node = None;
        for value in values {
            head_node = Option::Some(Box::from(ListNode::new(value, head_node)));
        }

        return head_node;
    }

}

fn main() {
    println!("{}", Solution::is_valid(String::from("{[]")));
}

