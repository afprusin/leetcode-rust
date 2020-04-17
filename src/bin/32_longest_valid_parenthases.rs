pub struct Solution {}

impl Solution {

    pub fn longest_valid_parentheses(s: String) -> i32 {
        let first_valid = s.find('(');
        if first_valid.is_none() {
            return 0;
        }

        let parenthesis: Vec<char> = s.chars().collect();
        let mut total: i32 = 0;
        let mut best_length: i32 = 0;
        let mut base_index: usize = first_valid.unwrap();
        let mut i: usize = first_valid.unwrap();
        loop {
            if i >= parenthesis.len() {
                break;
            }
            let current: char = parenthesis[i];
            match current {
                '(' => total += 1,
                ')' => total -= 1,
                _ => panic!("Input contained invalid characters")
            }
            if total == 0 {
                best_length = i32::max(best_length, ((i - base_index) + 1) as i32);
                if i + 1 < parenthesis.len() && parenthesis[i + 1] != '(' {
                    while i + 1 < parenthesis.len() && parenthesis[i + 1] != '(' {
                        i += 1;
                    }
                    base_index = i + 1;
                }
            }
            i += 1;
        }
        return best_length;
    }
}

fn main() {
    assert_eq!(2, Solution::longest_valid_parentheses("(()".to_string()));
    assert_eq!(4, Solution::longest_valid_parentheses(")()())".to_string()));
    assert_eq!(2, Solution::longest_valid_parentheses("()(()".to_string()));

}

