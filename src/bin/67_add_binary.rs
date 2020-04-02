pub struct Solution {}

impl Solution {
    pub fn get_bin_character_val(binary_character: char) -> i32 {
        if binary_character == '1' {
            return 1;
        }
        return 0;
    }

    pub fn add_binary(a: String, b: String) -> String {
        let mut has_carry: bool = false;
        let mut iter_a = a.chars().into_iter().rev().peekable();
        let mut iter_b = b.chars().into_iter().rev().peekable();
        let mut result = String::new();

        while !iter_a.peek().is_none() || !iter_b.peek().is_none() {
            let value_a = match iter_a.next() {
                Some(a_char) => Solution::get_bin_character_val(a_char),
                None => 0
            };
            let value_b = match iter_b.next() {
                Some(b_char) => Solution::get_bin_character_val(b_char),
                None => 0
            };
            let mut total = value_a + value_b;
            if has_carry {
                total += 1;
            }
            if total >= 2 {
                has_carry = true;
                total -= 2;
            }
            else {
                has_carry = false;
            }
            result = total.to_string() + &result;
        }
        if has_carry {
            result = 1.to_string() + &result;
        }

        return result;
    }
}

fn main() {
    assert_eq!(String::from("10101"), Solution::add_binary(String::from("1010"), String::from("1011")));
    assert_eq!(String::from("100"), Solution::add_binary(String::from("1"), String::from("11")));
}
