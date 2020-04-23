pub struct Solution {}

impl Solution {

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let length = nums.len();
        let mut bottom: usize = 0;
        let mut top: usize = length;

        while bottom != top {
            let midpoint = ((top - bottom) / 2) + bottom;
            let bottom_value = nums[bottom];
            let top_value = nums[top - 1];
            let value = nums[midpoint];

            if target == value {
                return midpoint as i32;
            }

            // If the bottom half is numerically continuous
            if value > bottom_value {
                if target >= bottom_value && target <= value {
                    top = midpoint
                } else {
                    bottom = midpoint;
                }
            }
            // Otherwise inspect the top half
            else {
                if target >= value && target <= top_value {
                    bottom = midpoint;
                } else {
                    top = midpoint
                }
            }
        }

        return -1;
    }

}

fn main() {
    assert_eq!(-1, Solution::search(vec![], 9));
    println!("-----");
    assert_eq!(-1, Solution::search(vec![1], 9));
    println!("-----");
    assert_eq!(0, Solution::search(vec![9], 9));
    println!("-----");
    assert_eq!(1, Solution::search(vec![9, 1], 1));
    println!("-----");
    assert_eq!(2, Solution::search(vec![7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7], 9));
    println!("-----");
    assert_eq!(-1, Solution::search(vec![7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7], 11));
    println!("-----");
    assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
}
