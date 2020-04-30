// TODO: New problem for contest; rename with correct problem number when added to the common list
use std::collections::{HashSet};

// TODO: I really need to implement some sort of linked hash set for future problems
struct FirstUnique {
    elements: Vec<i32>,
    contains: HashSet<i32>,
    is_duplicate: HashSet<i32>
}

impl FirstUnique {

    fn new(nums: Vec<i32>) -> Self {
        let mut new = FirstUnique {
            elements: Vec::new(),
            contains: HashSet::new(),
            is_duplicate: HashSet::new()
        };

        for number in nums {
            FirstUnique::add(&mut new, number)
        }

        return new;
    }

    fn show_first_unique(&self) -> i32 {
        for i in 0..self.elements.len() {
            if !self.is_duplicate.contains(&self.elements[i]) {
                return self.elements[i];
            }
        }
        return -1;
    }

    fn add(&mut self, value: i32) {
        if self.contains.insert(value) {
            self.elements.push(value);
        } else {
            self.is_duplicate.insert(value);
        }
    }
}

/**
 * Your FirstUnique object will be instantiated and called as such:
 * let obj = FirstUnique::new(nums);
 * let ret_1: i32 = obj.show_first_unique();
 * obj.add(value);
 */

fn main() {
    let mut first_unique: FirstUnique = FirstUnique::new(vec![2, 3, 5]);
    assert_eq!(2, first_unique.show_first_unique()); // return 2
    first_unique.add(5);            // the queue is now [2,3,5,5]
    assert_eq!(2, first_unique.show_first_unique()); // return 2
    first_unique.add(2);            // the queue is now [2,3,5,5,2]
    assert_eq!(3, first_unique.show_first_unique()); // return 3
    first_unique.add(3);            // the queue is now [2,3,5,5,2,3]
    assert_eq!(-1, first_unique.show_first_unique()); // return -1
}