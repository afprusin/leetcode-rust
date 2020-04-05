pub struct Solution {}

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut current_mult_five: i64 = 5;
        let mut trailing_zeros: i64 = 0;
        let n_long: i64 = n as i64;

        while current_mult_five <= n_long {
            trailing_zeros += n_long / current_mult_five;
            current_mult_five *= 5;
        }

        return trailing_zeros as i32;
    }
}

fn main() {
    assert_eq!(0, Solution::trailing_zeroes(1));
    assert_eq!(0, Solution::trailing_zeroes(3));
    assert_eq!(1, Solution::trailing_zeroes(5));
    assert_eq!(3, Solution::trailing_zeroes(16));
    assert_eq!(249, Solution::trailing_zeroes(1000));
    assert_eq!(452137076, Solution::trailing_zeroes(1808548329));
}