struct Solution {}

impl Solution {

    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut total: i32 = 0;
        let mut best_length: i32 = 0;
        // Using positive and negative vectors is very fiddly, but saves hashmap lookup times
        let mut previous_windows_negative: Vec<i32> = Vec::new();
        previous_windows_negative.push(0);
        let mut previous_windows_positive: Vec<i32> = Vec::new();
        previous_windows_positive.push(0);

        for i in 0..nums.len() {
            // Convert 0 to -1 to make checks easier
            match nums[i] {
                0 => total -= 1,
                1 => total += 1,
                _ => panic!("Array contained invalid data")
            }

            if total == 0 {
                // If the whole array (at this index) is balanced, assume best length
                best_length = i as i32 + 1;
            } else {
                let previous_windows: &mut Vec<i32>;
                if total < 0 {
                    previous_windows = &mut previous_windows_negative;
                } else {
                    previous_windows = &mut previous_windows_positive;
                }
                match previous_windows.get(i32::abs(total) as usize) {
                    Some(total_index) => {
                        // Total previously encountered, check length with that window subtracted
                        best_length = i32::max(best_length, i as i32 - *total_index);
                    },
                    None => {
                        // First time this total has been encountered, store index
                        previous_windows.push(i as i32);
                    }
                }
            }
        }
        return best_length;
    }
}

fn main() {
    assert_eq!(2, Solution::find_max_length(vec![0, 1]));
    assert_eq!(2, Solution::find_max_length(vec![0, 1, 0]));
    assert_eq!(6, Solution::find_max_length(vec![0, 0, 1, 0, 0, 0, 1, 1]));
}