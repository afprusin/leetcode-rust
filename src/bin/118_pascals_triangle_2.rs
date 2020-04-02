pub struct Solution {}

impl Solution {
    pub fn get_row(num_rows: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        if num_rows == 0 {
            result = vec![1];
        }
        if num_rows >= 1 {
            result = vec![1, 1];
        }
        for row in 2..num_rows + 1 {
            let mut current_row: Vec<i32> = Vec::new();
            current_row.push(1);
            for index in 0..(row - 1) {
                current_row.push(result[index as usize] + result[(index + 1) as usize]);
            }
            current_row.push(1);
            result = current_row;
        }

        return result;
    }
}

fn main() {
    assert_eq!(vec![1], Solution::get_row(0));
    assert_eq!(vec![1, 1], Solution::get_row(1));
    assert_eq!(vec![1, 2, 1], Solution::get_row(2));
}
