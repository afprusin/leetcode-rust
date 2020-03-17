pub struct Solution {}

impl Solution {

    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut input: i32 = x;
        let mut digits: Vec<i32> = vec![];

        while input > 0 {
            digits.push(input % 10);
            input /= 10;
        }

        let length: usize = digits.len();
        for i in 0 .. length / 2 {
            if digits[i] != digits[length - (i + 1)] {
                return false;
            }
        }

        return true;
    }

}

fn main() {
    println!("{}", Solution::is_palindrome(123));
}

