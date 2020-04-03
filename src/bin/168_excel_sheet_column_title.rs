pub struct Solution {}

impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        static UPPERCASE: [char; 26] = ['A','B','C','D','E','F','G','H','I','J','K','L',
            'M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
        let mut current_value = n;
        let mut chars: Vec<char> = Vec::new();

        while current_value > 0 {
            current_value -= 1;
            let char_value = current_value % 26;
            current_value /= 26;
            chars.push(UPPERCASE[(char_value) as usize]);
        }

        return chars.into_iter().rev().collect();
    }
}

fn main() {
    assert_eq!("A", Solution::convert_to_title(1));
    assert_eq!("B", Solution::convert_to_title(2));
    assert_eq!("Z", Solution::convert_to_title(26));
    assert_eq!("AA", Solution::convert_to_title(27));
    assert_eq!("AB", Solution::convert_to_title(28));
}