use std::collections::HashMap;

struct Solution {}

impl Solution {

    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut current_sum = 0;
        let mut sum_counts: HashMap<i32, i32> = HashMap::new();
        sum_counts.insert(0, 1);
        let mut match_count = 0;
        for i in 0..nums.len() {
            current_sum += nums[i];
            let to_find = current_sum - k;
            if let Some(&count) = sum_counts.get(&to_find) {
                match_count += count;
            }

            *sum_counts.entry(current_sum).or_insert(0) += 1;
        }

        match_count
    }
}

fn main() {
    assert_eq!(0, Solution::subarray_sum(vec![], 1));
    assert_eq!(1, Solution::subarray_sum(vec![1], 1));
    assert_eq!(0, Solution::subarray_sum(vec![1], 9));
    assert_eq!(3, Solution::subarray_sum(vec![1, 1, 1], 1));
    assert_eq!(2, Solution::subarray_sum(vec![1, 1, 1], 2));
    assert_eq!(1, Solution::subarray_sum(vec![-1, -1, 1], 0));
}