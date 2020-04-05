use std::cmp::Ordering::{Less, Equal, Greater};
use std::borrow::BorrowMut;

pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let zero_last_compare = |l: &i32, r: &i32| {
            return match (l, r) {
                (0, 0) => Equal,
                (0, _) => Greater,
                (_, 0) => Less,
                (_, _) => Equal
            }
        };
        nums.sort_by(zero_last_compare);
    }
}

fn main() {
    let mut input1 = vec![0];
    Solution::move_zeroes(input1.borrow_mut());
    assert_eq!(vec![0], input1);

    let mut input2 = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(input2.borrow_mut());
    assert_eq!(vec![1,3,12,0,0], input2);

    let mut input2 = vec![2, 1];
    Solution::move_zeroes(input2.borrow_mut());
    assert_eq!(vec![2, 1], input2);
}