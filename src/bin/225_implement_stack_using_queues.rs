struct MyStack {
    stack: Vec<i32>
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            stack: Vec::new()
        }
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        return self.stack.pop().unwrap();
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        return self.stack[self.stack.len() - 1]
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        return self.stack.is_empty();
    }
}

fn main() {
    let mut stack = MyStack::new();

    stack.push(7);
    assert_eq!(7, stack.top());
    assert_eq!(7, stack.pop());
    assert_eq!(true, stack.empty());
}