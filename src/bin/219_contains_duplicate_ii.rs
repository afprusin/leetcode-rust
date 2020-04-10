use std::collections::{HashSet};

pub struct Solution {}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut exists: HashSet<i32> = HashSet::new();
        let expired_index_offset = k + 1;

        for i in 0 .. nums.len() {
            if i as i32 - expired_index_offset >= 0 {
                exists.remove(&nums[i - expired_index_offset as usize]);
            }
            if !exists.insert(nums[i]) {
                return true;
            }
        }

        return false;
    }
}

fn main() {
    assert_eq!(false, Solution::contains_nearby_duplicate(vec![1, 2 ,4, 6, 7], 20));
    assert_eq!(false, Solution::contains_nearby_duplicate(Vec::new(), 1));
    assert_eq!(false, Solution::contains_nearby_duplicate(vec![0], 1));
    assert_eq!(false, Solution::contains_nearby_duplicate(vec![0, 1], 1));
    assert_eq!(false, Solution::contains_nearby_duplicate(vec![1,2,3,1,2,3],2));
    assert_eq!(true, Solution::contains_nearby_duplicate(vec![1,0,1,1],1));
}