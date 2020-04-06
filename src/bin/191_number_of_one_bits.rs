pub struct Solution {}

impl Solution {
    // Leetcode specifies method name which is not snake-case
    pub fn hammingWeight (n: u32) -> i32 {
        return format!("{:b}", n).chars().into_iter()
            .filter(|bit| bit == &'1')
            .count() as i32;
    }
}

fn main() {
    assert_eq!(1, Solution::hammingWeight(1));
    assert_eq!(1, Solution::hammingWeight(2));
    assert_eq!(2, Solution::hammingWeight(3));
    assert_eq!(1, Solution::hammingWeight(4));
    assert_eq!(3, Solution::hammingWeight(6));
}