use std::borrow::BorrowMut;

pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 1;
        loop {
            if i >= nums.len() {
                break;
            }
            if nums.get(i) == nums.get(i - 1) {
                nums.remove(i);
            }
            else {
                i += 1;
            }
        }
        return nums.len() as i32;
    }
}

fn main() {
    let mut input = vec![1,1,2,3,4,4,4,4,5];
    let result = Solution::remove_duplicates(input.borrow_mut());
    println!("{}", result);
    for num in input {
        println!("{}", num);
    }
}
