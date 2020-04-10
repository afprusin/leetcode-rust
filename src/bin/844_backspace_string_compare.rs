pub struct Solution {}

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        return get_typed_string(s) == get_typed_string(t);

        fn get_typed_string(to_type: String) -> Vec<char> {
            let mut result: Vec<char> = Vec::new();
            for character in to_type.chars() {
                if character == '#' {
                    result.pop();
                } else {
                    result.push(character);
                }
            }
            return result;
        }
    }
}

fn main() {
    assert_eq!(true, Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()));
    assert_eq!(true, Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()));
    assert_eq!(true, Solution::backspace_compare("a##c".to_string(), "#a#c".to_string()));
    assert_eq!(false, Solution::backspace_compare("a#c".to_string(), "b".to_string()));
}