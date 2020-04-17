use std::collections::{HashMap};

pub struct Solution {}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut exists: HashMap<i32, i32> = nums1.iter().fold(HashMap::new(), |mut acc, &c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let mut current = nums2.iter();
        let mut result: Vec<i32> = Vec::new();

        while let Some(&value) = current.next() {
            if let Some(&count) = exists.get(&value) {
                if count > 0 {
                    exists.insert(value, count - 1);
                    result.push(value);
                }
            }
        }
        return result;
    }
}

fn main() {
    assert_eq!(vec![2, 2], Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]));
    assert_eq!(vec![9, 4], Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]));
}