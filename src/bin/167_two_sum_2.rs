pub struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut low = 0;
        let mut high = numbers.len() - 1;

        loop {
            let needed_diff = target - numbers[low];
            while numbers[high] > needed_diff {
                high -= 1;
            }
            if numbers[high] == needed_diff {
                break;
            }
            low += 1;
        }

        return vec![low as i32 + 1, high as i32 + 1];
    }
}

fn main() {
    assert_eq!(vec![1, 2], Solution::two_sum(vec![0, 1], 1));
    assert_eq!(vec![2, 3], Solution::two_sum(vec![0, 1, 2], 3));
}