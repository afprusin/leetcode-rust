use std::collections::VecDeque;

// TODO: New problem for contest; rename with correct problem number when added to the common list
struct Solution {}

impl Solution {
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        if s.is_empty() {
            return s;
        }
        let mut total: i32 = 0;

        for instruction in shift {
            let direction = match instruction[0] {
                0 => -1,
                1 => 1,
                val => panic!("Invalid shift value in vector (expected 1 or 0): {}", val)
            };
            total += direction * instruction[1];
        }

        let mut chars: VecDeque<char> = s.chars().collect();
        if total > 0 {
            let rotate_right: usize = (total % chars.len() as i32) as usize;
            chars.rotate_right(rotate_right);

        } else if total < 0 {
            let rotate_left: usize = i32::abs(total % chars.len() as i32) as usize;
            chars.rotate_left(rotate_left)
        }

        return chars.iter().collect();
    }
}

fn main() {
    assert_eq!("cab".to_string(), Solution::string_shift("abc".to_string(), vec![vec![0,1], vec![1,2]]));
    assert_eq!("efgabcd".to_string(), Solution::string_shift("abcdefg".to_string(), vec![vec![1,1], vec![1,1], vec![0,2], vec![1,3]]));
}