pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current_best: i32 = nums[0];

        // I'd imagine there's a better way to do this
        for i in 0..nums.len() {
            let mut current_total = 0;
            for j in i..nums.len() {
                current_total += nums[j];
                if current_total > current_best {
                    current_best = current_total;
                }
            }
        }

        return current_best;
    }
}

fn main() {
    //println!("{}", Solution::max_sub_array(vec![-2,-3,-1]));
    println!("{}", Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
}
