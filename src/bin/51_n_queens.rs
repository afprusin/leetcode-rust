pub struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        if n <= 0 {
            return vec![];
        }
        let diagonals = (n * 2 - 1) as usize;
        let mut results: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut columns_used: Vec<bool> = vec![false; n as usize];
        let mut right_diagonals_used = vec![false; diagonals];
        let mut left_diagonals_used = vec![false; diagonals];
        let mut coords: Vec<(usize, usize)> = Vec::new();

        Solution::search_recursively(
            &mut columns_used,
            &mut right_diagonals_used,
            &mut left_diagonals_used,
            &mut coords,
            &mut results,
            0,
            n as usize);

        return results.iter()
            .map(|result| Solution::get_as_printed(&result, n as usize))
            .collect();
    }

    fn search_recursively(
        columns_used: &mut Vec<bool>,
        right_diagonals_used: &mut Vec<bool>,
        left_diagonals_used: &mut Vec<bool>,
        coords: &mut Vec<(usize, usize)>,
        results: &mut Vec<Vec<(usize, usize)>>,
        row: usize,
        queens: usize) {

        if row >= queens {
            results.push(coords.clone());
            return;
        }

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

                Solution::search_recursively(
                    columns_used,
                    right_diagonals_used,
                    left_diagonals_used,
                    coords,
                    results,
                    row + 1,
                    queens);

                coords.pop();
                columns_used[column] = false;
                left_diagonals_used[l_diag_index] = false;
                right_diagonals_used[r_diag_index] = false;
            }
        }
    }

    fn get_as_printed(coordinates: &Vec<(usize, usize)>, columns: usize) -> Vec<String> {
        let blank_line: Vec<char> = vec!['.'; columns];
        let mut as_printed: Vec<String> = Vec::new();

        for coord in coordinates {
            let mut current_line = blank_line.clone();
            current_line[coord.1] = 'Q';
            as_printed.push(current_line.iter().collect());
        }

        return as_printed;
    }
}

fn main() {

    assert_eq!(Vec::<Vec<String>>::new(), Solution::solve_n_queens(0));
    assert_eq!(vec![vec!["Q".to_string()]], Solution::solve_n_queens(1));
    assert_eq!(Vec::<Vec<String>>::new(), Solution::solve_n_queens(2));

}
