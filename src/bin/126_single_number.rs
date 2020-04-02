use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut values: HashSet<i32> = HashSet::new();

        for num in nums {
            if !values.insert(num) {
                values.remove(&num);
            }
        }

        return *values.iter().next().unwrap();
    }
}

fn main() {
    assert_eq!(1, Solution::single_number(vec![1,2,2,3,3,4,4]));
}