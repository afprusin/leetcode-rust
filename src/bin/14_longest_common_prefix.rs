pub struct Solution {}

impl Solution {

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::from("");
        }
        let mut input: Vec<String> = strs.to_vec();
        let mut shortest_length = input[0].len();
        let mut shortest_index = 0;

        for i in 1 .. input.len() {
            if input[i].len() < shortest_length {
                shortest_length = input[i].len();
                shortest_index = i;
            }
        }

        let shortest_input: Vec<char> = input.remove(shortest_index).chars().collect();
        let mut common_length = shortest_input.len();
        for current_input in &input {
            let current_chars: Vec<char> = current_input.chars().collect();
            if current_chars.len() < common_length {
                common_length = current_chars.len();
            }
            for i in 0 .. common_length {
                if shortest_input[i] != current_chars[i] {
                    common_length = i;
                    break;
                }
            }
            if common_length == 0 {
                break;
            }
        }

        return shortest_input[..common_length].into_iter().collect();
    }

}

fn main() {
    println!("{}", Solution::longest_common_prefix(vec![
        String::from("a")]));
}

