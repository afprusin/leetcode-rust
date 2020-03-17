pub struct Solution {}

impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + i)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        panic!("A valid pair was not found")
    }
}

fn main() {
    let v = vec![2, 7, 11, 15];
    let result = Solution::two_sum(v, 9);
    println!("Hello, world!");
    for (position, element) in result.iter().enumerate() {
        println!("Element at position {}: {:?}", position, element);
    }
}

