use std::borrow::BorrowMut;

pub struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.len() == 0 || nums.len() == 1 {
            return;
        }
        let rotations = k % nums.len() as i32;
        if rotations == 0 {
            return;
        }

        // Store the last values that would be 'rotated out'
         let mut buffer: Vec<i32> = Vec::new();
        for i in nums.len() - rotations as usize .. nums.len() {
            buffer.push(nums[i]);
        }
        // Shift values forward
        let start_relocation_index = nums.len() - rotations as usize;
        for i in (0..start_relocation_index).rev()  {
            nums[i + rotations as usize] = nums[i];
        }
        // Copy stored values to head of vector
        for i in 0 .. rotations as usize {
            nums[i] = buffer[i];
        }
    }
}

fn main() {
    let mut input: Vec<i32> = vec![];
    Solution::rotate(input.borrow_mut(), 1);
    assert_eq!(Vec::<i32>::new(), input);

    let mut input = vec![1];
    Solution::rotate(input.borrow_mut(), 1);
    assert_eq!(vec![1], input);

    let mut input = vec![1,2,3,4,5,6,7];
    Solution::rotate(input.borrow_mut(), 1);
    assert_eq!(vec![7,1,2,3,4,5,6], input);

    let mut input = vec![1,2,3,4,5,6,7];
    Solution::rotate(input.borrow_mut(), 2);
    assert_eq!(vec![6,7,1,2,3,4,5], input);

    let mut input = vec![1,2,3,4,5,6,7];
    Solution::rotate(input.borrow_mut(), 7);
    assert_eq!(vec![1,2,3,4,5,6,7], input);
}