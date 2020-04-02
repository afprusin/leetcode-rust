pub struct Solution {}

impl Solution {
    // Cribbed from user MinhTuanTran's post in the discussion section - a very elegant
    // Fibonacci sequence, and a good introduction to the Fold operator's syntax
    pub fn climb_stairs(n: i32) -> i32 {
        (0..n).fold((0, 1), |acc, _| (acc.1, acc.0 +  acc.1)).1
    }
}

fn main() {
    assert_eq!(2, Solution::climb_stairs(2));
    assert_eq!(3, Solution::climb_stairs(3));
}
