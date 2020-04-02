pub struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        return (x as f64).sqrt() as i32;
    }
}

fn main() {
    assert_eq!(2, Solution::my_sqrt(4));
    assert_eq!(2, Solution::my_sqrt(8));
    assert_eq!(46339, Solution::my_sqrt(2147395599));
}
