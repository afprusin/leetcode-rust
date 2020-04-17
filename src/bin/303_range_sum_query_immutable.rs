struct NumArray {
    numbers: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        NumArray {
            numbers: nums
        }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        if i < 0 {
            panic!("Lower bounds out of range");
        } else if j as usize >= self.numbers.len() {
            panic!("Upper bounds out of range");
        }
        return *(&self.numbers[i as usize ..= j as usize].iter().sum());
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(i, j);
 */

fn main() {
    let n = NumArray::new(vec![1,2,3,4,5,6,7]);
    assert_eq!(2, n.sum_range(1, 1));
    assert_eq!(5, n.sum_range(1, 2));
}