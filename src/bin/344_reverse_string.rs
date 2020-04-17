pub struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        return s.reverse();
    }
}

fn main() {
    assert_eq!(vec!['a', 'b', 'c'], Solution::reverse_string($mut vec!['c', 'b', 'a']));
}