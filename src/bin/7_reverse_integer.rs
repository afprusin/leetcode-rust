pub struct Solution {}

impl Solution {

    pub fn reverse(x: i32) -> i32 {
        let mut to_reverse: i32 = x;
        let mut reversed: i32 = 0;

        while to_reverse != 0 {
            reversed = match reversed.checked_mul(10) {
                Some(val) => val,
                None => { return 0; },
            };
            reversed += to_reverse % 10;
            to_reverse /= 10;
        }

        return reversed;
    }

}

fn main() {
    println!("{}", Solution::reverse(1534236469));
}

