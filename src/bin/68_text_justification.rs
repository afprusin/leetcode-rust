pub struct Solution {}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut lines: Vec<Vec<&String>> = Vec::new();

        let mut current_line = Vec::new();
        let mut character_count = 0;
        for word in &words {
            if (character_count + word.len() + current_line.len()) as i32 <= max_width {
                current_line.push(word);
                character_count += word.len();
            } else {
                lines.push(current_line);
                current_line = Vec::new();
                current_line.push(word);
                character_count = word.len();
            }
        }

        return vec![];
    }
}

fn main() {
    assert_eq!(Vec::<String>::new(), Solution::full_justify(vec![], 0));
}
