pub struct Solution {}

impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        let mut digits: Vec<i32> = Vec::new();
        while num > 0 {
            let digit = num % 10;
            if digit != 0 {
                digits.push(digit);
            }
            num /= 10;
        }
        let mut result = 0;
        while !digits.is_empty() {
            result += digits.pop().unwrap();
            if result >= 10 {
                result -= 9;
            }
        }
        return result;
    }
}

fn main() {
    assert_eq!(2, Solution::add_digits(38));
    assert_eq!(2, Solution::add_digits(11));
    assert_eq!(1, Solution::add_digits(19));
}