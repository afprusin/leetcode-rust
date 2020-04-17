pub struct Solution {}

impl Solution {

    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 1;
        }
        let mut numbers: Vec<bool> = vec![false; nums.len()];

        let input_length = nums.len() as i32;
        for i in nums {
            if i > 0 && i <= input_length {
                numbers[(i - 1) as usize] = true;
            }
        }
        for i in 0..numbers.len() {
            if !numbers[i] {
                return (i + 1) as i32;
            }
        }

        return (numbers.len() + 1) as i32;
    }
}

fn main() {
    assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 0]));
    assert_eq!(2, Solution::first_missing_positive(vec![3, 4, -1, 1]));
    assert_eq!(1, Solution::first_missing_positive(vec![7, 8, 9, 11, 12]));

}
