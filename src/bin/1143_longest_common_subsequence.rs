use std::collections::{HashSet};

pub struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut chars_a: Vec<char> = text1.chars().collect();
        let mut chars_b: Vec<char> = text2.chars().collect();
        if text1.len() < text2.len() {
            let shortest_chars: HashSet<char> = text1.chars().collect();
            chars_b = chars_b.iter()
                .filter(|&character| shortest_chars.contains(character))
                .cloned()
                .collect();
        } else {
            let shortest_chars: HashSet<char> = text2.chars().collect();
            chars_a = chars_a.iter()
                .filter(|&character| shortest_chars.contains(character))
                .cloned()
                .collect();
        }

        let length_b = chars_b.len();
        let mut current_values: Vec<usize> = vec![0; length_b + 1];
        let mut next_values: Vec<usize> = vec![0; length_b + 1];

        for character_a in chars_a {
            let tmp = current_values;
            current_values = next_values;
            next_values = tmp;
            for (j, &character_b) in chars_b.iter().enumerate() {
                if character_a == character_b {
                    next_values[j + 1] = current_values[j] + 1;
                } else {
                    next_values[j + 1] = usize::max(next_values[j], current_values[j + 1]);
                }
            }
        }
        return next_values[length_b] as i32;
    }
}

fn main() {
    assert_eq!(3, Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()));
    assert_eq!(3, Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()));
    assert_eq!(0, Solution::longest_common_subsequence("abc".to_string(), "def".to_string()));
    assert_eq!(1, Solution::longest_common_subsequence("bl".to_string(), "yby".to_string()));
}
