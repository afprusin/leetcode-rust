pub struct Solution {}

impl Solution {
    // Would be better to cache results, but let's brute-force it
    pub fn is_power_of_two(n: i32) -> bool {
        let mut power = 0;
        let base: i64 = 2;
        loop {
            let result = base.pow(power);
            if result == n as i64 {
                return true;
            }
            else if result > n as i64 {
                break;
            }
            power += 1;
        }
        return false;
    }
}

fn main() {
    assert_eq!(true, Solution::is_power_of_two(1));
    assert_eq!(true, Solution::is_power_of_two(16));
    assert_eq!(false, Solution::is_power_of_two(218));
}