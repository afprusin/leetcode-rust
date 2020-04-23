pub struct Solution {}

impl Solution {

    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.is_empty() || nums.len() == 1 {
            return 0;
        }

        let last_index = nums.len() - 1;
        let mut current_index: usize = 0;
        let mut jumps = 0;
        while current_index < last_index {
            if current_index + nums[current_index] as usize >= last_index {
                jumps += 1;
                break;
            }

            let mut best_length: usize = 0;
            let mut jump_to = 0;
            for i in 1..=nums[current_index] as usize {
                let jump_length = i + nums[i + current_index] as usize;
                if jump_length > best_length {
                    best_length = jump_length;
                    jump_to = current_index + i;
                }
            }
            jumps += 1;
            current_index = jump_to;
        }

        return jumps;
    }

}

fn main() {
    assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
    assert_eq!(2, Solution::jump(vec![1, 2, 3]));

}
