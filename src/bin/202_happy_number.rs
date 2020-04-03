use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    // There probably exists a way to identify numbers that will loop without iteration
    pub fn is_happy(n: i32) -> bool {
        static MAX_ITERATIONS: i32 = 100;
        let mut previous_values: HashSet<i32> = HashSet::new();
        let mut current_value: i32 = n;
        let mut iteration_count = 0;

        while current_value != 1 &&
            !previous_values.contains(&current_value) &&
            iteration_count < MAX_ITERATIONS {

            previous_values.insert(current_value);
            let digits: Vec<i32> = to_digits(current_value);
            current_value = digits.iter().map(|&x| x * x).sum();
            iteration_count += 1;
        }

        return current_value == 1;

        fn to_digits(mut whole_number: i32) -> Vec<i32> {
            let mut digits: Vec<i32> = Vec::new();
            while whole_number > 0 {
                digits.push(whole_number % 10);
                whole_number = whole_number / 10;
            }
            return digits;
        }
    }
}

fn main() {
    assert_eq!(false, Solution::is_happy(0));
    assert_eq!(true, Solution::is_happy(19));
}