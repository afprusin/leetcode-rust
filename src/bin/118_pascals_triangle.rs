pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        if num_rows > 0 {
            result.push(vec![1]);
        }
        if num_rows > 1 {
            result.push(vec![1, 1]);
        }
        for row in 2..num_rows {
            let previous_row: &Vec<i32> = &result[(row - 1) as usize];
            let mut current_row: Vec<i32> = Vec::new();
            current_row.push(1);
            for index in 0..(row - 1) {
                current_row.push(previous_row[index as usize] + previous_row[(index + 1) as usize]);
            }
            current_row.push(1);
            result.push(current_row);
        }

        return result;
    }
}

fn main() {
    assert_eq!(Vec::<Vec<i32>>::new(), Solution::generate(0));
    assert_eq!(vec![vec![1]], Solution::generate(1));
    assert_eq!(vec![vec![1], vec![1, 1]], Solution::generate(2));
    assert_eq!(vec![vec![1], vec![1, 1], vec![1, 2, 1]], Solution::generate(3));
}
