pub struct Solution {}

impl Solution {
    // TODO: Grid of 'used' spaces?
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let rows = matrix.len();
        let columns = matrix[0].len();

        let mut max_dimension = 0;
        for row in 0..rows {
            if row + max_dimension >= rows {
                break;
            }
            for column in 0..columns {
                if column + max_dimension >= columns {
                    continue
                }
                if matrix[row][column] == '1' {
                    max_dimension = usize::max(max_dimension, Solution::explore_recursively(
                        &matrix, row, column, 0));
                }
            }
        }

        return (max_dimension * max_dimension) as i32;
    }

    fn explore_recursively(matrix: &Vec<Vec<char>>, top_row: usize,
                           top_column: usize, dimension: usize) -> usize {
        let row = top_row + dimension;
        for i in top_column..=(top_column + dimension) {
            if matrix[row][i] != '1' {
                return 0;
            }
        }
        let column = top_column + dimension;
        for i in top_row..(top_row + dimension) {
            if matrix[i][column] != '1' {
                return 0;
            }
        }

        let next_dimension = dimension + 1;
        return if top_row + next_dimension < matrix.len() &&
            top_column + next_dimension < matrix[0].len() {
            1 + Solution::explore_recursively(matrix, top_row, top_column, next_dimension)
        } else {
            1
        }
    }
}

fn main() {
    assert_eq!(0, Solution::maximal_square(vec![]));
    assert_eq!(1, Solution::maximal_square(vec![vec!['1']]));
    assert_eq!(4, Solution::maximal_square(vec![vec!['1', '1'], vec!['1', '1']]));
    assert_eq!(1, Solution::maximal_square(vec![vec!['1', '0'], vec!['1', '0']]));
    assert_eq!(1, Solution::maximal_square(vec![vec!['0', '0'], vec!['0', '1']]));
    assert_eq!(0, Solution::maximal_square(vec![vec!['0', '0'], vec!['0', '0']]));
    assert_eq!(9, Solution::maximal_square(vec![
        vec!['1', '1', '0', '0'],
        vec!['1', '1', '1', '1'],
        vec!['0', '1', '1', '1'],
        vec!['0', '1', '1', '1']]));
    assert_eq!(1, Solution::maximal_square(vec![
        vec!['0','0','0','1','0','1','1','1'],
        vec!['0','1','1','0','0','1','0','1'],
        vec!['1','0','1','1','1','1','0','1'],
        vec!['0','0','0','1','0','0','0','0'],
        vec!['0','0','1','0','0','0','1','0'],
        vec!['1','1','1','0','0','1','1','1'],
        vec!['1','0','0','1','1','0','0','1'],
        vec!['0','1','0','0','1','1','0','0'],
        vec!['1','0','0','1','0','0','0','0']]));
}
