pub struct Solution {}

impl Solution {

    pub fn roman_to_int(s: String) -> i32 {

        let mut total: i32 = 0;
        let characters: Vec<char> = s.chars().collect();

        for i in 0 .. characters.len() {
            if characters[i] == 'M' {
                total += 1000;
            }
            else if characters[i] == 'D' {
                total += 500;
            }
            else if characters[i] == 'C' {
                if Solution::has_follower(&characters, i, vec!['D', 'M']) {
                    total -= 100;
                }
                else {
                    total += 100;
                }
            }
            else if characters[i] == 'L' {
                total += 50;
            }
            else if characters[i] == 'X' {
                if Solution::has_follower(&characters, i, vec!['L', 'C']) {
                    total -= 10;
                }
                else {
                    total += 10;
                }
            }
            else if characters[i] == 'V' {
                total += 5;
            }
            else if characters[i] == 'I' {
                if Solution::has_follower(&characters, i, vec!['V', 'X']) {
                    total -= 1;
                }
                else {
                    total += 1;
                }
            }
        }

        return total;
    }

    pub fn has_follower(letters: &Vec<char>, index: usize, followers: Vec<char>) -> bool {
        if (index + 1) < letters.len() {
            for follower in followers {
                if follower == letters[index + 1] {
                    return true;
                }
            }
        }
        return false;
    }

}

fn main() {
    println!("{}", Solution::roman_to_int(String::from("MCMXCIV")));
}

