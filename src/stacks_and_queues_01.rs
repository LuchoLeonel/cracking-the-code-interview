use std::cmp::min;

pub fn run() {
    #[derive(Debug)]
    struct Stack {
        vector: Vec<i32>,
        min: Vec<i32>,
    }

    impl Stack {
        fn new() -> Stack {
            Stack {
                vector: Vec::new(),
                min: Vec::new()
            }
        }

        fn push(&mut self, value: i32) {
            let last_min = self.min.last().unwrap_or(&value);
            let min = min(value, *last_min);
            self.min.push(min);
            self.vector.push(value);
        }

        fn pop(&mut self) {
            self.vector.pop();
            self.min.pop();
        }

        fn min(&self) -> Option<i32> {
            self.min.last().copied()
        }

    }

    fn print(result: Option<i32>) {
        if let Some(n) = result {
            print!("Min: {}\n", n);
        } else {
            print!("Empty stack\n");
        }
    }

    let mut stack = Stack::new();
    assert_eq!(stack.min(), None);
    print(stack.min());

    stack.push(3);
    stack.push(66);
    stack.push(7);
    stack.push(88);
    stack.pop();
    stack.pop();
    assert_eq!(stack.min(), Some(3));
    print(stack.min());

    stack.push(1);
    assert_eq!(stack.min(), Some(1));
    print(stack.min());
    
    stack.push(0);
    assert_eq!(stack.min(), Some(0));
    print(stack.min());
    
    stack.pop();
    assert_eq!(stack.min(), Some(1));
    print(stack.min());
    
    stack.pop();
    assert_eq!(stack.min(), Some(3));
    print(stack.min());
    
}