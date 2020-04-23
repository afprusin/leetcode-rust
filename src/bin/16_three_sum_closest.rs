pub struct Solution {}

impl Solution {

    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut best_sum = nums[0] + nums[1] + nums[2];
        let mut best_distance = i32::abs(best_sum - target);

        //TODO: Bad input checks?
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let pair_value = nums[i] + nums[j];

                let needed = target - pair_value;
                match nums.binary_search(&needed) {
                    Ok(index) | Err(index) => {
                        let low;
                        if index < 2 {
                            low = 0;
                        } else {
                            low = index - 2;
                        }
                        let high;
                        if index + 2 >= nums.len() {
                            high = nums.len() - 1;
                        } else {
                            high = index + 2;
                        };

                        for k in low..=high {
                            if k == i || k == j {
                                continue;
                            }
                            let this_sum = pair_value + nums[k];
                            let this_distance = i32::abs(target - this_sum);
                            if this_distance < best_distance {
                                best_sum = this_sum;
                                best_distance = this_distance;
                            }
                        }
                    }
                }
            }
        }
        return best_sum;
    }

}

fn main() {

    assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    println!();
    assert_eq!(3, Solution::three_sum_closest(vec![1, 1, 1, 0], 100));

}
