pub struct Solution {}

impl Solution {

    pub fn longest_valid_parentheses(s: String) -> i32 {
        let first_valid = s.find('(');
        if first_valid.is_none() {
            return 0;
        }

        let mut positive_totals: Vec<i32> = vec![0];
        let mut negative_totals: Vec<i32> = vec![0];

        let mut total: i32 = 0;
        let mut best_length: i32 = 0;
        let parenthesis = &s[first_valid.unwrap()..s.len()];

        for (i, current) in parenthesis.chars().enumerate() {
            match current {
                '(' => total += 1,
                ')' => total -= 1,
                _ => panic!("Input contained invalid characters")
            }
            if total == 0 {
                best_length = (i + 1) as i32;
            } else {
                let totals = match total {
                    x if x < 0 => &mut negative_totals,
                    _ => &mut positive_totals
                };
                if totals.len() as i32 <= i32::abs(total) {
                    totals.push(i as i32);
                } else {
                    let previous_window = totals[i32::abs(total) as usize];
                    best_length = i32::max(best_length, i as i32 - previous_window);
                }
            }
        }
        return best_length;
    }
}

fn main() {
    assert_eq!(2, Solution::longest_valid_parentheses("(()".to_string()));
    assert_eq!(4, Solution::longest_valid_parentheses(")()())".to_string()));
}

