use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_t_relations: HashMap<char, char> = HashMap::new();
        let mut t_s_relations: HashMap<char, char> = HashMap::new();
        let mut s_chars = s.chars();
        let mut t_chars = t.chars();

        while let (Some(i), Some(j)) = (s_chars.next(), t_chars.next()) {
            match (s_t_relations.get(&i), t_s_relations.get(&j)) {
                (Some(_), None) | (None, Some(_)) => return false,
                (None, None) => {
                    s_t_relations.insert(i, j);
                    t_s_relations.insert(j, i);
                },
                (Some(t_val), Some(s_val)) => {
                    if *s_val != i || *t_val != j {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}

fn main() {
    assert_eq!(true, Solution::is_isomorphic("egg".to_string(), "add".to_string()));
    assert_eq!(false, Solution::is_isomorphic("foo".to_string(), "bar".to_string()));
    assert_eq!(true, Solution::is_isomorphic("paper".to_string(), "title".to_string()));
    assert_eq!(false, Solution::is_isomorphic("ab".to_string(), "aa".to_string()));
}