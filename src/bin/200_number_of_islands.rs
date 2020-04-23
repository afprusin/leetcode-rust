use std::borrow::BorrowMut;

pub struct Solution {}

impl Solution {

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }

        let mut grid = grid;
        let mut islands: i32 = 0;

        let row_length = grid.len();
        let column_length = grid[0].len();
        for row_index in 0..row_length {
            for column_index in 0..column_length {
                match grid[row_index][column_index] {
                    '1' => {
                        explore_recursively(row_index, column_index, grid.borrow_mut());
                        islands += 1;
                    },
                    _ => {}
                }
            }
        }

        return islands;

        fn explore_recursively(row: usize, column: usize, grid: &mut Vec<Vec<char>>) {
            let mut to_explore: Vec<(usize, usize)> = vec![(row, column)];
            grid[row][column] = '0';

            while let Some(current) = to_explore.pop() {
                let current_row = current.0;
                let current_column = current.1;

                if current_row > 0 && current_row < (grid.len() - 1) {
                    if grid[current_row - 1][current_column] != '0' {
                        grid[current_row - 1][current_column] = '0';
                        to_explore.push((current_row - 1, current_column));
                    }
                    if grid[current_row + 1][current_column] != '0' {
                        grid[current_row + 1][current_column] = '0';
                        to_explore.push((current_row + 1, current_column));
                    }
                }
                if current_column > 0 && current_column < (grid[0].len() - 1) {
                    if grid[current_row][current_column - 1] != '0' {
                        grid[current_row][current_column - 1] = '0';
                        to_explore.push((current_row, current_column - 1));
                    }
                    if  grid[current_row][current_column - 1] != '0' {
                        grid[current_row][current_column + 1] = '0';
                        to_explore.push((current_row, current_column + 1));
                    }
                }
            }
        }
    }
}

fn main() {
    assert_eq!(1, Solution::num_islands(vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0']
    ]));

    assert_eq!(3, Solution::num_islands(vec![
        "11000".chars().collect(),
        "11000".chars().collect(),
        "00100".chars().collect(),
        "00011".chars().collect()
    ]));
}
