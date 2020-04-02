pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut lowest_cost = prices[0];
        let mut most_profit = 0;
        for i in 1..prices.len() {
            if prices[i] < lowest_cost {
                lowest_cost = prices[i];
            }
            else if prices[i] > lowest_cost {
                let profit = prices[i] - lowest_cost;
                if profit > most_profit {
                    most_profit = profit;
                }
            }
        }

        return most_profit;
    }
}

fn main() {
    assert_eq!(5, Solution::max_profit(vec![7,1,5,3,6,4]));
}
