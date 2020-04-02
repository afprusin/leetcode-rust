pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut count: i32 = 0;
        for current_char in s.trim().chars().rev() {
            if current_char == ' ' {
                break;
            }
            count += 1;
        }
        return count;
    }
}

fn main() {
    assert_eq!(5, Solution::length_of_last_word(String::from("Hello World")));
    assert_eq!(1, Solution::length_of_last_word(String::from("a ")));
}
