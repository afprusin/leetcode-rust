use std::collections::BTreeMap;

pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count_index: BTreeMap<i32, i32> = BTreeMap::new();

        for number in nums {
            *count_index.entry(number).or_insert(0) += 1;
        }
        return *count_index.iter().max_by_key(|&(_, count)| count).unwrap().0;
    }
}

fn main() {
    assert_eq!(3, Solution::majority_element(vec![3, 2, 3]));
    assert_eq!(1, Solution::majority_element(vec![1, 2, 1, 2, 1]));
}