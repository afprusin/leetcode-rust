pub struct Solution {}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        let diagonals = (n * 2 - 1) as usize;
        let mut columns_used: Vec<bool> = vec![false; n as usize];
        let mut right_diagonals_used = vec![false; diagonals];
        let mut left_diagonals_used = vec![false; diagonals];
        let mut coords: Vec<(usize, usize)> = Vec::new();

        return Solution::search_recursively(
            &mut columns_used,
            &mut right_diagonals_used,
            &mut left_diagonals_used,
            &mut coords,
            0,
            n as usize);
    }

    fn search_recursively(
        columns_used: &mut Vec<bool>,
        right_diagonals_used: &mut Vec<bool>,
        left_diagonals_used: &mut Vec<bool>,
        coords: &mut Vec<(usize, usize)>,
        row: usize,
        queens: usize) -> i32 {

        if row >= queens {
            return 1;
        }

        let mut solution_totals = 0;

        for column in 0..queens {
            let l_diag_index = row + column;
            let r_diag_index = (queens - 1) - row + column;
            if !columns_used[column] &&
                !left_diagonals_used[l_diag_index] &&
                !right_diagonals_used[r_diag_index] {
                coords.push((row, column));
                columns_used[column] = true;
                left_diagonals_used[l_diag_index] = true;
                right_diagonals_used[r_diag_index] = true;

                solution_totals += Solution::search_recursively(
                    columns_used,
                    right_diagonals_used,
                    left_diagonals_used,
                    coords,
                    row + 1,
                    queens);

                coords.pop();
                columns_used[column] = false;
                left_diagonals_used[l_diag_index] = false;
                right_diagonals_used[r_diag_index] = false;
            }
        }
        return solution_totals;
    }
}

fn main() {

    assert_eq!(0, Solution::total_n_queens(0));
    assert_eq!(1, Solution::total_n_queens(1));
    assert_eq!(0, Solution::total_n_queens(2));

}
