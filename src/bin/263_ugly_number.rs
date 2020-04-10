pub struct Solution {}

impl Solution {
    pub fn is_ugly(mut num: i32) -> bool {
        if num == 0 {
            return false;
        }

        for i in vec![2, 3, 5] {
            while num % i == 0 {
                num /= i;
            }
        }

        return num == 1;
    }
}

fn main() {
    assert_eq!(true, Solution::is_ugly(6));
    assert_eq!(true, Solution::is_ugly(8));
    assert_eq!(false, Solution::is_ugly(14));
    assert_eq!(false, Solution::is_ugly(-2147483648));
    assert_eq!(false, Solution::is_ugly(0));
}