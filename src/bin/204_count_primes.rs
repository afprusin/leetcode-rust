pub struct Solution {}

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut count: i32 = 0;

        for i in 2..n {
            if is_prime(i) {
                count += 1;
            }
        }

        return count;

        // From user Alexandru's post on S.O. - simplified form of O(sqrt(N)) algorithm
        fn is_prime(number: i32) -> bool {
            // Returns True if n is prime.
            if number == 2 {
                return true;
            }
            if number == 3 {
                return true;
            }
            if number % 2 == 0 {
                return false
            }
            if number % 3 == 0 {
                return false
            }

            let mut i: i32 = 5;
            let mut w: i32 = 2;

            while i * i <= number {
                if number % i == 0 {
                    return false;
                }
                i += w;
                w = 6 - w;
            }

            return true;
        }
    }
}

fn main() {
    assert_eq!(4, Solution::count_primes(10));
    assert_eq!(41537, Solution::count_primes(499979));
}