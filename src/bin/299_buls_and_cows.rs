use std::collections::HashMap;

pub struct Solution {}

// TODO: This is sorta messy; I'm guessing there is a cleaner way to do it with a one-map
//  two-iterator rollup sort of approach
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut secret_chars = secret.chars().peekable();
        let mut guess_chars = guess.chars();
        let mut position_matches = 0;
        let mut secret_counts: HashMap<char, i32> = HashMap::new();
        let mut guess_counts: HashMap<char, i32> = HashMap::new();

        while secret_chars.peek().is_some() {
            let secret_char = secret_chars.next().unwrap();
            let guess_char = guess_chars.next().unwrap();
            if secret_char == guess_char {
                position_matches += 1;
            } else {
                *secret_counts.entry(secret_char).or_insert(0) += 1;
                *guess_counts.entry(guess_char).or_insert(0) += 1;
            }
        }

        let value_matches: i32 = secret_counts.iter()
            .map(|entry| i32::min(
                *entry.1,
                match guess_counts.get(entry.0) {
                    None => 0,
                    Some(count) => *count
                }))
            .sum();

        return format!("{}A{}B", position_matches, value_matches);
    }
}

fn main() {
    assert_eq!("1A3B".to_string(), Solution::get_hint("1807".to_string(), "7810".to_string()));
    assert_eq!("1A1B".to_string(), Solution::get_hint("1123".to_string(), "0111".to_string()));
}