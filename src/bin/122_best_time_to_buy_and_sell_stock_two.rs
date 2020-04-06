pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut last = prices[0];
        let mut total = 0;

        for i in 1 .. prices.len() {
            let current = prices[i];
            if current > last {
                total += current - last;
            }
            last = current;
        }

        return total;
    }
}

fn main() {
    assert_eq!(7, Solution::max_profit(vec![7,1,5,3,6,4]));
    assert_eq!(4, Solution::max_profit(vec![1,2,3,4,5]));
    assert_eq!(0, Solution::max_profit(vec![7,6,4,3,1]));
}
