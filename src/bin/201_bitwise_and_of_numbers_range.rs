pub struct Solution {}

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut base: i64 = 1;
        let m: i64 = m as i64;
        let n: i64 = n as i64;
        let mut total = 0;

        while base <= m && base <= n {
            if ((m / base) % 2 != 0) && ((n / base) % 2 != 0) && (n - m < base) {
                total += base;
            }
            base *= 2;
        }

        return total as i32;
    }
}

fn main() {
    assert_eq!(4, Solution::range_bitwise_and(5, 7));
    assert_eq!(0, Solution::range_bitwise_and(0, 1));
    assert_eq!(2147483647, Solution::range_bitwise_and(2147483647, 2147483647));
    assert_eq!(1, Solution::range_bitwise_and(1, 1));

}
