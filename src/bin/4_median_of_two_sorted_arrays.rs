pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut both: Vec<i32> = Vec::from(nums1);
        both.extend(nums2.iter().cloned());
        both.sort();

        let half_length = both.len() / 2;
        return if both.len() % 2 == 0 {
            (both[half_length] + both[half_length - 1]) as f64 / 2 as f64
        } else {
            both[half_length] as f64
        }
    }
}

fn main() {
    assert_eq!(1.5, Solution::find_median_sorted_arrays(vec![1], vec![2]));
    assert_eq!(2.5, Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
}

