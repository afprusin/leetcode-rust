pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut svec: Vec<char> = s.chars().collect();
        let mut tvec: Vec<char> = t.chars().collect();
        svec.sort();
        tvec.sort();

        return svec == tvec;
    }
}

fn main() {
    assert_eq!(true, Solution::is_anagram("anagram".to_string(), "nagaram".to_string()));
    assert_eq!(false, Solution::is_anagram("rat".to_string(), "car".to_string()));
}