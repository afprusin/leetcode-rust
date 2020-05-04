struct Solution {}

impl Solution {

    pub fn candy(ratings: Vec<i32>) -> i32 {

        let mut rating_positions: Vec<(i32, i32)> = Vec::new();
        for (position, &rating) in ratings.iter().enumerate() {
            rating_positions.push((rating, position as i32));
        }
        rating_positions.sort();

        let mut candies: Vec<i32> = vec![1; ratings.len()];

        for (rating, position) in rating_positions {
            let position_left = position - 1;
            let mut candies_left = 0;
            if position_left >= 0 && ratings[position_left as usize] < rating {
                candies_left = candies[position_left as usize];
            }

            let position_right: usize = (position + 1) as usize;
            let mut candies_right = 0;
            if position_right < ratings.len() && ratings[position_right] < rating {
                candies_right = candies[position_right];
            }

            candies[position as usize] =
                i32::max(candies_left, candies_right) + 1;
        }

        return candies.iter().sum();
    }

}

fn main() {
    assert_eq!(0, Solution::candy(vec![]));
    assert_eq!(5, Solution::candy(vec![1,0,2]));
    assert_eq!(4, Solution::candy(vec![1,2,2]));
}