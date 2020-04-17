use std::iter::FromIterator;

pub struct Solution {}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut characters: Vec<char> = s.chars().collect();
        let mut i: i32 = 0;
        let mut j: i32 = (characters.len() - 1) as i32;

        loop {
            while i < characters.len() as i32 && !Solution::is_vowel(characters[i as usize]) {
                i += 1;
            }
            while j >= 0 && !Solution::is_vowel(characters[j as usize]) {
                j -= 1;
            }
            if i >= j || j < 0 || i > characters.len() as i32 {
                break;
            }
            let swap = characters[i as usize];
            characters[i as usize] = characters[j as usize];
            characters[j as usize] = swap;
            i += 1;
            j -= 1;
        }
        return String::from_iter(characters);
    }

    fn is_vowel(to_check: char) -> bool {
        // TODO: Assuming basic latin characters
        let lowercase = to_check.to_lowercase().next().unwrap();
        return lowercase == 'a' || lowercase == 'e' || lowercase == 'i' ||
            lowercase == 'o' || lowercase == 'u';
    }
}

fn main() {
    assert_eq!("leotcede".to_string(), Solution::reverse_vowels("leetcode".to_string()));
    assert_eq!("Aa".to_string(), Solution::reverse_vowels("aA".to_string()));
}