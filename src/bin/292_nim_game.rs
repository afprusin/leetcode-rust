pub struct Solution {}

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        if n <= 0 {
            return true;
        }
        // 'Four stones remaining' is the first scenario where a player cannot win within one turn.
        //  If the board is not a multiple of four, the opponent may keep the board at a
        //  multiple of four, after their turn, until only four stones remain.
        return !(n % 4 == 0);
    }
}

fn main() {
    assert_eq!(true, Solution::can_win_nim(1));
    assert_eq!(true, Solution::can_win_nim(3));
    assert_eq!(false, Solution::can_win_nim(4));
    assert_eq!(false, Solution::can_win_nim(8));
    assert_eq!(true, Solution::can_win_nim(5));
    assert_eq!(true, Solution::can_win_nim(9));
}