use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        if pattern.is_empty() {
            return str.is_empty();
        }

        let mut bindings: HashMap<char, String> = HashMap::new();
        let mut used_words: HashSet<String> = HashSet::new();
        let mut words_iter = str.split_whitespace().peekable();

        for binding in pattern.chars() {
            match words_iter.next() {
                None => return false,
                Some(word) => {
                    if used_words.contains(word) && !bindings.contains_key(&binding) {
                        return false;
                    } else {
                        used_words.insert(word.to_string());
                    }
                    let bound_word = bindings.entry(binding).or_insert(word.to_string());
                    if bound_word != word {
                        return false;
                    }
                }
            }
        }
        if words_iter.peek().is_some() {
            return false;
        }
        return true;
    }
}

fn main() {
    assert_eq!(true, Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()));
    assert_eq!(false, Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string()));
    assert_eq!(false, Solution::word_pattern("aaaa".to_string(), "dog cat cat fish".to_string()));
    assert_eq!(false, Solution::word_pattern("abba".to_string(), "dog dog dog dog".to_string()));
    assert_eq!(false, Solution::word_pattern("".to_string(), "dog".to_string()));
    assert_eq!(false, Solution::word_pattern("a".to_string(), "".to_string()));
    assert_eq!(true, Solution::word_pattern("".to_string(), "".to_string()));
}