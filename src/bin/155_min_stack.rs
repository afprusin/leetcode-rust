struct MinStack {
    stack: Vec<i32>
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new()
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&mut self) -> i32 {
        return self.stack[self.stack.len() - 1];
    }

    fn get_min(&self) -> i32 {
        return *self.stack.iter().min().unwrap();
    }
}

fn main() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(-3, min_stack.get_min());
    min_stack.pop();
    assert_eq!(0, min_stack.top());
    assert_eq!(-2, min_stack.get_min());
}