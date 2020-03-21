use std::borrow::BorrowMut;

pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        loop {
            if i >= nums.len() {
                break;
            }
            // TODO: Need to research appropriate usages for Unsafe
            if unsafe { nums.get_unchecked(i) } == &val {
                nums.remove(i);
            } else {
                i += 1;
            }
        }
        return nums.len() as i32;
    }
}

fn main() {
    let mut result = Solution::remove_element(vec![1, 1, 2, 3, 4, 4, 4, 4, 5].borrow_mut(), 4);
}
