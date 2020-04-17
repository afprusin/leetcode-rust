pub struct Solution {}

impl Solution {
    // Would be better to cache results, but let's brute-force it
    pub fn is_power_of_three(n: i32) -> bool {
        let mut current: i32 = 1;
        while current <= n {
            if current == n {
                return true;
            }
            let next = current.checked_mul(3);
            if next.is_none() {
                return false;
            }
            current = next.unwrap();
        }

        return false;
    }
}

fn main() {
    assert_eq!(true, Solution::is_power_of_three(3));
    assert_eq!(true, Solution::is_power_of_three(9));
    assert_eq!(false, Solution::is_power_of_three(12));
    assert_eq!(false, Solution::is_power_of_three(2147483647));
}