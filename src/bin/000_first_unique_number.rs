// TODO: New problem for contest; rename with correct problem number when added to the common list
use std::collections::{HashMap};

struct OrderNode {
    previous: OrderNode,
    next: OrderNode
}

impl OrderNode {
    pub fn new() -> Self {
        OrderNode {
            previous: self,
            next: self
        }
    }
}


struct FirstUnique {
    elements: Vec<i32>,
    element_index: HashMap<i32, (usize, OrderNode)>
}

impl FirstUnique {

    fn new(nums: Vec<i32>) -> Self {
        let mut first_node = OrderNode::new();
        let mut new = FirstUnique {
            elements: Vec::new(),
            element_index: HashMap::new()
        };
        for number in nums {
            FirstUnique::add(&mut new, number)
        }
        return new;
    }

    fn show_first_unique(&self) -> i32 {
        match self.elements.iter().next() {
            Some(&element) => element,
            None => -1
        }
    }

    fn add(&mut self, value: i32) {
        let entry = self.element_index.entry(value)
            .or_insert((self.elements.len(), 0));
        entry.1 += 1;

        if entry.1 == 1 {
            self.elements.push(value);
        } else if entry.1 == 2 {
            let start = usize::min(self.elements.len() - 1, entry.0);
            for i in (0..=start).rev() {
                if self.elements[i] == value {
                    self.elements.remove(i);
                    break;
                }
            }
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