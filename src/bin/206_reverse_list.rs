use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();
        return nums.iter().any(|number| !seen.insert(*number));
    }
}

fn main() {
    assert_eq!(false, Solution::contains_duplicate(vec![]));
    assert_eq!(false, Solution::contains_duplicate(vec![1]));
    assert_eq!(false, Solution::contains_duplicate(vec![1, 2, 99]));
    assert_eq!(true, Solution::contains_duplicate(vec![1, 2, 99, 1]));
}