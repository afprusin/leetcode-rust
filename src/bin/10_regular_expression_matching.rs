extern crate regex;
use regex::Regex;

pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let re = Regex::new(format!("^{}$", p)).unwrap();
        return re.is_match(s);
    }
}

fn main() {
    assert_eq!(false, Solution::is_match("aa".to_string(), "a".to_string()));
    assert_eq!(true, Solution::is_match("aa".to_string(), "a*".to_string()));
    assert_eq!(true, Solution::is_match("aas".to_string(), "c*a*b*".to_string()));
    assert_eq!(false, Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()));
}

