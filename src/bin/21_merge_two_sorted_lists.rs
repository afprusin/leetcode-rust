pub struct Solution {}

impl Solution {

    pub fn is_valid(s: String) -> bool {
        let brackets: Vec<char> = s.chars().collect();
        let mut order_stack: Vec<char> = Vec::new();

        for bracket in brackets {
            if bracket == '(' || bracket == '[' || bracket == '{' {
                order_stack.push(bracket);
            }
            else {
                match order_stack.pop() {
                    None => { return false; },
                    Some('(') => if bracket != ')' { return false; }
                    Some('{') => if bracket != '}' { return false; }
                    Some('[') => if bracket != ']' { return false; }
                    _ => panic!("Invalid closing bracket")
                };
            }
        }
        return order_stack.is_empty();
    }

}

fn main() {
    println!("{}", Solution::is_valid(String::from("{[]")));
}

