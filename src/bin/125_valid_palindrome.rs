pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let characters: Vec<char> = s.chars().collect();
        let mut front_index: i32 = 0;
        let mut rear_index: i32 = (characters.len() - 1) as i32;

        loop {
            front_index = get_next_front_character_index(front_index, &characters);
            rear_index = get_next_rear_character_index(rear_index, &characters);
            if front_index >= rear_index ||
                front_index >= characters.len() as i32 ||
                rear_index < 0 {
                return true;
            }
            if !characters[front_index as usize].eq_ignore_ascii_case(&characters[rear_index as usize]) {
                return false;
            }
            front_index += 1;
            rear_index -= 1;
        }

        fn get_next_front_character_index(mut current: i32, characters: &Vec<char>) -> i32 {
            while current < characters.len() as i32 &&
                !characters[current as usize].is_alphanumeric() {
                current += 1;
            }
            return current;
        }

        fn get_next_rear_character_index(mut current: i32, characters: &Vec<char>) -> i32 {
            while current >= 0 && !characters[current as usize].is_alphanumeric() {
                current -= 1;
            }
            return current;
        }
    }
}

fn main() {
    assert_eq!(true, Solution::is_palindrome("".to_string()));
    assert_eq!(true, Solution::is_palindrome(" ".to_string()));
    assert_eq!(true, Solution::is_palindrome("a".to_string()));
    assert_eq!(false, Solution::is_palindrome("ab".to_string()));
    assert_eq!(true, Solution::is_palindrome("aa".to_string()));
    assert_eq!(true, Solution::is_palindrome("aba".to_string()));
    assert_eq!(true, Solution::is_palindrome("a ba".to_string()));
    assert_eq!(true, Solution::is_palindrome("a.ba".to_string()));
    assert_eq!(true, Solution::is_palindrome("aba-".to_string()));
    assert_eq!(true, Solution::is_palindrome("racecar".to_string()));
}