pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        return match nums.binary_search(&target) {
            Ok(pos) | Err(pos) => pos as i32
        };
    }
}

fn main() {
    println!("{}", Solution::search_insert(vec![1,2,3], 2));
}
