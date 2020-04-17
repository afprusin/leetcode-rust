use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub struct Solution {}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.is_empty() || words.is_empty() {
            return Vec::new();
        }
        let word_length = words[0].len() as i32;
        let mut indexes_by_word: HashMap<&String, HashSet<i32>> = HashMap::new();

        // Get starting indexes for each word
        for word in HashSet::<&String>::from_iter(words.iter()) {
            let mut indexes: HashSet<i32> = HashSet::new();
            for i in 0..s.len() {
                if s[i..].starts_with(word) {
                    indexes.insert(i as i32);
                }
            }
            indexes_by_word.insert(word, indexes);
        }

        // Get counts of words
        let mut word_counts: HashMap<String, i32> = HashMap::new();
        for word in &words {
            let entry = word_counts.entry(word.clone()).or_insert(0);
            *entry += 1;
        }
        let mut count_entries: Vec<(String, i32)> = Vec::new();
        for (word, count) in word_counts {
            count_entries.push((word, count));
        }

        let mut positions: Vec<i32> = Vec::new();
        let substring_length = word_length * words.len() as i32;
        for i in 0..=(s.len() as i32 - substring_length) {
            if match_recursively(i, word_length, &count_entries, &indexes_by_word) {
                positions.push(i as i32);
            }
        }

        return positions;

        fn match_recursively(current_index: i32, length: i32,
                             remaining_words: &Vec<(String, i32)>,
                             indexes: &HashMap<&String, HashSet<i32>>) -> bool {
            if remaining_words.is_empty() {
                return true;
            }

            for i in 0..remaining_words.len() {
                let entry = &remaining_words[i];
                let word = &entry.0;

                if indexes.get(word).unwrap().contains(&current_index) {
                    let mut next_remaining = remaining_words.clone();
                    next_remaining[i].1 -= 1;
                    if next_remaining[i].1 <= 0 {
                        next_remaining.remove(i);
                    }

                    if match_recursively(current_index + length, length,
                                         &next_remaining, &indexes) {
                        return true;
                    }
                }
            }
            return false;
        }
    }
}

fn main() {
    assert_eq!(true, "banana".starts_with("banana"));
    assert_eq!(vec![0, 9], Solution::find_substring(
        "barfoothefoobarman".to_string(), vec!["foo".to_string(), "bar".to_string()]));
    assert_eq!(vec![8], Solution::find_substring(
        "wordgoodgoodgoodbestword".to_string(), vec!["word".to_string(), "good".to_string(), "best".to_string(), "good".to_string()]));
    assert_eq!(vec![0, 1, 2], Solution::find_substring(
        "aaaaaaaa".to_string(), vec!["aa".to_string(), "aa".to_string(), "aa".to_string()]));
}

