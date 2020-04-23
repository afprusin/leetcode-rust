pub struct Solution {}

impl Solution {

    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        // Processing data in-place
        let mut grid = grid;
        let x_length = grid.len() as i32;
        let y_length = grid[0].len() as i32;

        let iterations = x_length + y_length - 1;
        for current_iteration in 0..iterations {
            let mut current_x = i32::min(current_iteration, x_length - 1);
            let mut current_y = i32::max(0, current_iteration - x_length + 1);
            while current_x >=0 && current_y < y_length {
                let x = current_x as usize;
                let y = current_y as usize;
                let cell_value = grid[x][y];
                if current_x - 1 >= 0 && current_y - 1 >= 0 {
                    grid[x][y] = cell_value + i32::min(grid[x - 1][y], grid[x][y - 1])
                } else if current_x - 1 >= 0 {
                    grid[x][y] = cell_value + grid[x - 1][y];
                } else if current_y - 1 >= 0 {
                    grid[x][y] = cell_value + grid[x][y - 1];
                }
                current_x -= 1;
                current_y += 1;
            }
        }
        return grid[(x_length - 1) as usize][(y_length - 1) as usize];
    }

}

fn main() {
    assert_eq!(0, Solution::min_path_sum(Vec::new()));
    assert_eq!(9, Solution::min_path_sum(vec![vec![9]]));
    assert_eq!(19, Solution::min_path_sum(vec![vec![9, 10]]));
    assert_eq!(42, Solution::min_path_sum(vec![vec![9, 10, 11, 12]]));

    assert_eq!(7, Solution::min_path_sum(vec![
        vec![1,3,1],
        vec![1,5,1],
        vec![4,2,1]]));

    assert_eq!(85, Solution::min_path_sum(vec![
        vec![7,1,3,5,8,9,9,2,1,9,0,8,3,1,6,6,9,5],
        vec![9,5,9,4,0,4,8,8,9,5,7,3,6,6,6,9,1,6],
        vec![8,2,9,1,3,1,9,7,2,5,3,1,2,4,8,2,8,8],
        vec![6,7,9,8,4,8,3,0,4,0,9,6,6,0,0,5,1,4],
        vec![7,1,3,1,8,8,3,1,2,1,5,0,2,1,9,1,1,4],
        vec![9,5,4,3,5,6,1,3,6,4,9,7,0,8,0,3,9,9],
        vec![1,4,2,5,8,7,7,0,0,7,1,2,1,2,7,7,7,4],
        vec![3,9,7,9,5,8,9,5,6,9,8,8,0,1,4,2,8,2],
        vec![1,5,2,2,2,5,6,3,9,3,1,7,9,6,8,6,8,3],
        vec![5,7,8,3,8,8,3,9,9,8,1,9,2,5,4,7,7,7],
        vec![2,3,2,4,8,5,1,7,2,9,5,2,4,2,9,2,8,7],
        vec![0,1,6,1,1,0,0,6,5,4,3,4,3,7,9,6,1,9]
    ]))

}
