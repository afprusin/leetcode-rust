use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut weights: BinaryHeap<i32> = BinaryHeap::from(stones);

        while weights.len() > 1 {
            let stone_one = weights.pop().unwrap();
            let stone_two = weights.pop().unwrap();
            if stone_one != stone_two {
                weights.push(i32::abs(stone_one - stone_two));
            }
        }

        return match weights.pop() {
            Some(weight) => weight,
            None => 0
        };
    }
}

fn main() {
    assert_eq!(1, Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]));
}