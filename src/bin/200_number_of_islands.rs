use std::collections::{HashSet};

pub struct Solution {}

impl Solution {

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }

        let mut islands: i32 = 0;
        let mut explored: HashSet<(usize, usize)> = HashSet::new();

        for (row_index, row) in grid.iter().enumerate() {
            for (column_index, terrain_type) in row.iter().enumerate() {
                match terrain_type {
                    '0' => {},
                    '1' => {
                        if !explored.contains(&(row_index, column_index)) {
                            explore_recursively(row_index, column_index, &grid, &mut explored);
                            islands += 1;
                        }
                    },
                    _ => panic!("Input contained invalid character")
                }
            }
        }

        return islands;

        fn explore_recursively(row: usize, column: usize,
                               grid: &Vec<Vec<char>>, explored: &mut HashSet<(usize, usize)>) {
            if grid[row][column] == '0' || !explored.insert((row, column)) {
                return;
            }

            if row > 0 {
                explore_recursively(row - 1, column, grid, explored);
            }
            if row < grid.len() - 1 {
                explore_recursively(row + 1, column, grid, explored);
            }
            if column > 0 {
                explore_recursively(row, column - 1, grid, explored);
            }
            if column < grid[0].len() - 1 {
                explore_recursively(row, column + 1, grid, explored);
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
