use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut has_carry = true;
        let mut plus_one_digits: VecDeque<i32> = VecDeque::new();
        for digit in digits.iter().rev() {
            let to_push: i32;
            if has_carry {
                to_push = digit + 1;
            }
            else {
                to_push = *digit;
            }
            if to_push == 10 {
                plus_one_digits.push_front(0)
            }
            else {
                has_carry = false;
                plus_one_digits.push_front(to_push);
            }
        }
        if has_carry {
            plus_one_digits.push_front(1);
        }

        return Vec::from(plus_one_digits);
    }
}

fn main() {
    assert_eq!(vec![4,3,2,2], Solution::plus_one(vec![4,3,2,1]));
    assert_eq!(vec![1,0], Solution::plus_one(vec![9]));
}
