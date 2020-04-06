use std::collections::HashMap;
use std::string::String;

fn main() {
    let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
    println!("{:?}", group_anagrams(strs.iter().map(|x| x.to_string()).collect()));
}

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut h = HashMap::<String, Vec<String>>::new();
    for s in strs {
        h.entry(get_sorted(&s))
            .or_default()
            .push(s);
    }

    h.into_iter()
        .map(|(_, v)| v)
        .collect()
}

fn get_sorted(s: &str) -> String {
    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();
    chars.iter().collect()
}