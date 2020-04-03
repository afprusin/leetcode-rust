pub struct Solution {}

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut column_number = 0;
        s.chars().for_each(|c| {
            // Not crazy about char->int tomfoolery
            let current_value = (c as i32) - 64;
            column_number *= 26;
            column_number += current_value;
        });

        return column_number;
    }
}

fn main() {
    assert_eq!(1, Solution::title_to_number("A".to_string()));
    assert_eq!(26, Solution::title_to_number("Z".to_string()));
    assert_eq!(27, Solution::title_to_number("AA".to_string()));
}