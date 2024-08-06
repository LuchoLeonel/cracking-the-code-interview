pub fn run() {
    #[derive(Debug)]
    struct Stack {
        vec: Vec<i32>,
    }

    impl Stack {
        fn new() -> Stack {
            Stack { vec: Vec::new() }
        }
        fn display(&self) {
            for i in self.vec.iter() {
                print!("{} -> ", i);
            }
            print!("None\n");
        }

        fn push(&mut self, value: i32) {
            self.vec.push(value);
        }
        fn pop(&mut self) -> Option<i32> {
            self.vec.pop()
        }
    }

    let mut stack = Stack::new();
    for i in 0..7 {
        stack.push(i);
    }
    stack.display();
    stack.pop();
    stack.display();

    #[derive(Debug)]
    struct Queue {
        stack1: Stack,
        stack2: Stack,
    }

    impl Queue {
        fn new() -> Queue {
            let stack1 = Stack::new();
            let stack2 = Stack::new();
            Queue { stack1, stack2 }
        }
        fn display(&mut self) {
            self.from_stack1_to_stack2();
            for i in self.stack2.vec.iter() {
                print!("{} -> ", i);
            }
            print!("None\n");
        }

        fn from_stack1_to_stack2(&mut self) {
            if self.stack1.vec.is_empty() {
                return;
            }
            for _ in 0..self.stack1.vec.len() {
                self.stack2.push(self.stack1.vec.remove(0));
            }
        }

        fn push(&mut self, value: i32) {
            self.stack1.push(value);
        }
        fn pop(&mut self) -> Option<i32> {
            self.from_stack1_to_stack2();
            self.stack2.pop()
        }
    }

    let mut queue = Queue::new();
    for i in 0..10 {
        queue.push(i);
    }
    queue.display();
    assert_eq!(queue.pop(), Some(9));
    queue.display();
    queue.push(77);
    queue.display();
    assert_eq!(queue.pop(), Some(77));
}