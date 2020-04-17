use std::collections::VecDeque;

pub struct Solution {}

impl Solution {

    pub fn check_valid_string(s: String) -> bool {
        let mut wildcards: VecDeque<usize> = VecDeque::new();
        let mut open_brackets: Vec<usize> = Vec::new();

        for (position, character) in s.chars().enumerate() {
            match character {
                '*' => wildcards.push_back(position),
                '(' => open_brackets.push(position),
                ')' => {
                    match open_brackets.pop() {
                        None => {
                            match wildcards.pop_front() {
                                None => {
                                    return false
                                },
                                Some(_wild) => {}
                            }
                        },
                        Some(_bracket) => {}
                    }
                },
                invalid @ _ => panic!("Invalid character in input string: {}", invalid)
            }
        }

        while let Some(bracket_pos) = open_brackets.pop() {
            loop {
                match wildcards.pop_back() {
                    Some(wild_pos) => {
                        if wild_pos > bracket_pos {
                            break;
                        }
                    },
                    None => {
                        return false
                    }
                }
            }
        }

        return open_brackets.is_empty();
    }

}

fn main() {
    assert_eq!(true, Solution::check_valid_string("(*))".to_string()));
    assert_eq!(true, Solution::check_valid_string("(*)".to_string()));
    assert_eq!(true, Solution::check_valid_string("()".to_string()));
    assert_eq!(false, Solution::check_valid_string(")(".to_string()));
    assert_eq!(false, Solution::check_valid_string(")*(".to_string()));
    assert_eq!(true, Solution::check_valid_string("*".to_string()));
    assert_eq!(true, Solution::check_valid_string("".to_string()));
    assert_eq!(false, Solution::check_valid_string("(())((())()()(*)(*()(())())())()()((()())((()))(*".to_string()));
    assert_eq!(true, Solution::check_valid_string("()*()(()(*()(((())()()())*))()*()(*)(((*))(())(())((*()*(()(())()*(((*(**))((())*)(((()()))(())()))".to_string()));
}
