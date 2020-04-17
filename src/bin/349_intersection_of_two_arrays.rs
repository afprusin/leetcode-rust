use std::iter::FromIterator;
use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let a = HashSet::<i32>::from_iter(nums1.into_iter())
            .intersection(&HashSet::from_iter(nums2.into_iter()))
            .map(|&i| i)
            .collect();
        return a;
    }
}

fn main() {
    assert_eq!(HashSet::<i32>::from_iter(vec![2].into_iter()),
               HashSet::<i32>::from_iter(Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]).into_iter()));
    assert_eq!(HashSet::<i32>::from_iter(vec![4, 9].into_iter()),
               HashSet::<i32>::from_iter(Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]).into_iter()));
}