struct Solution {}

impl Solution {

    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // Hacky; more flexible to just use a map
        let mut available: Vec<i32> = vec![0; 26];

        for character in magazine.chars() {
            available[Solution::get_character_value(character)] += 1;
        }
        for character in ransom_note.chars() {
            let index = Solution::get_character_value(character);
            available[index] -= 1;
            if available[index] < 0 {
                return false;
            }
        }

        return true;
    }

    fn get_character_value(to_convert: char) -> usize {
        let a_offset = 'a' as usize;
        to_convert as usize - a_offset
    }

}

fn main() {
    assert_eq!(false, Solution::can_construct("a".to_string(), "b".to_string()));
    assert_eq!(false, Solution::can_construct("aa".to_string(), "ab".to_string()));
    assert_eq!(true, Solution::can_construct("aa".to_string(), "aab".to_string()));
}