use std::collections::HashSet;

struct Solution {}

impl Solution {

    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let jewels: HashSet<char> = j.chars().collect();
        return s.chars()
            .filter(|stone| jewels.contains(stone))
            .count() as i32;
    }

}

fn main() {
    assert_eq!(3, Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()));
    assert_eq!(0, Solution::num_jewels_in_stones("z".to_string(), "ZZZZZZZZ".to_string()));
}