// TODO: New problem for contest; rename with correct problem number when added to the common list
use std::collections::BTreeMap;

struct Solution {}

impl Solution {
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        if arr.is_empty() {
            return 0;
        }

        let mut counts: BTreeMap<i32, i32> = BTreeMap::new();
        for number in arr {
            *counts.entry(number).or_insert(0) += 1;
        }

        return counts.keys()
            .flat_map(|key| {
                if counts.contains_key(&(key + 1)) {
                    counts.get(key)
                } else {
                    None
                }})
            .sum();
    }
}

fn main() {
    assert_eq!(0, Solution::count_elements(vec![]));
    assert_eq!(2, Solution::count_elements(vec![1, 2, 3]));
    assert_eq!(0, Solution::count_elements(vec![1, 1, 3, 3, 5, 5, 7, 7]));
    assert_eq!(3, Solution::count_elements(vec![1, 3, 2, 3, 5, 0]));
    assert_eq!(2, Solution::count_elements(vec![1, 1, 2, 2]));
}