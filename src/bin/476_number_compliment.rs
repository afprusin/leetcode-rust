struct Solution {}

impl Solution {

    pub fn find_complement(num: i32) -> i32 {
        let mut number = num as i64;
        let mut powers_of_two: Vec<i64> = Vec::new();
        let mut current_power: i64 = 1;

        while current_power <= number {
            powers_of_two.push(current_power);
            current_power *= 2;
        }

        let mut inverted = 0;
        for &power in powers_of_two.iter().rev() {
            if power > number {
                inverted += power;
            } else {
                number -= power;
            }
        }

        return inverted as i32;
    }

}

fn main() {
    assert_eq!(2, Solution::find_complement(5));
    assert_eq!(0, Solution::find_complement(1));
    assert_eq!(1, Solution::find_complement(2));
}