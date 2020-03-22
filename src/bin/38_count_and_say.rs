pub struct Solution {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result: String = String::from("1");

        for _x in 1..n {
            result = Solution::get_as_say(result);
        }

        return result;
    }

    pub fn get_as_say(to_say: String) -> String {
        let mut last_char: char = to_say.chars().next().unwrap();
        let mut duplicate_count: i32 = 0;
        let mut char_iterator = to_say.chars().into_iter();
        let mut as_say: String = String::new();
        loop {
            match char_iterator.next() {
                None => {
                    as_say.push_str(duplicate_count.to_string().as_ref());
                    as_say.push(last_char);
                    break;
                },
                Some(current_char) => {
                    if last_char != current_char {
                        as_say.push_str(duplicate_count.to_string().as_ref());
                        as_say.push(last_char);
                        last_char = current_char;
                        duplicate_count = 1;
                    }
                    else {
                        duplicate_count += 1;
                    }
                }
            }
        }
        return as_say;
    }
}

fn main() {
    println!("{}", Solution::count_and_say(5));
}
