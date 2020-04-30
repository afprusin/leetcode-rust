pub struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return true;
        }
        let length = nums.len();
        let mut reachable = 0;
        for i in 0..(length - 1) {
            let current_cell_reachable = i as i32 + nums[i];
            if current_cell_reachable > reachable {
                reachable = current_cell_reachable;
            }
            if reachable <= i as i32 {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    assert_eq!(true, Solution::can_jump(vec![0]));
    assert_eq!(true, Solution::can_jump(vec![]));
    assert_eq!(true, Solution::can_jump(vec![1, 0]));
    assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4]));
    assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4]));
    assert_eq!(true, Solution::can_jump(vec![5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0]));
}
