pub fn run() {
    #[derive(Debug)]
    struct Stack {
        vec: Vec<i32>,
    }

    #[derive(Debug)]
    struct MinStack {
        main_stack: Stack,
        temporary_stack: Stack,
    }

    impl Stack {
        fn new() -> Stack {
            Stack { vec: Vec::new() }
        }

        fn push(&mut self, value: i32) {
            self.vec.push(value);
        }
        fn pop(&mut self) -> Option<i32> {
            self.vec.pop()
        }
        fn is_empty(&self) -> bool {
            self.vec.is_empty()
        }
    }

    impl MinStack {
        fn new() -> MinStack {
            MinStack {
                main_stack: Stack::new(),
                temporary_stack: Stack::new(),
            }
        }
        fn display(&self) {
            for v in self.main_stack.vec.iter().rev() {
                print!("{} -> ", v);
            }
            print!("None\n");
        }
        fn move_from_temporary_to_main(&mut self) {
            while !self.temporary_stack.is_empty() {
                self.main_stack.push(self.temporary_stack.pop().unwrap())
            }
        }

        fn push(&mut self, value: i32) {
            let len = self.main_stack.vec.len();
            if len < 1 {
                self.main_stack.push(value);
                return;
            }

            let mut pushed = false;
            for i in 0..len {
                let v = self.main_stack.pop().unwrap();
                self.temporary_stack.push(v);
                if v > value && !pushed || i+1 == len {
                    self.temporary_stack.push(value);
                    pushed = true;
                }
            }

            self.move_from_temporary_to_main();
            
        }
        fn pop(&mut self) -> Option<i32> {
            self.main_stack.pop()
        }
        fn is_empty(&self) -> bool {
            self.main_stack.is_empty()
        }
        fn peek(&mut self, index: i32) -> Option<i32> {
            if index >= self.main_stack.vec.len() as i32 {
                return None;
            }
            let mut value = None;
            for i in 0..index+1 {
                let v = self.main_stack.pop().unwrap();
                self.temporary_stack.push(v);
                if i == index {
                    value = Some(v);
                }
            }
            self.move_from_temporary_to_main();

            value
        }
    }

    let mut min_stack = MinStack::new();
    for i in 1..15 {
        min_stack.push(i);
        min_stack.display();
    }
    min_stack.display();
    print!("{}\n", min_stack.peek(8).unwrap());

}