pub fn run() {

    #[derive(Debug)]
    struct SetOfStacks {
        stacks: Vec<Stack>,
        limit: i32,
    }

    #[derive(Debug)]
    struct Stack {
        stack: Vec<i32>,
    }

    impl SetOfStacks {
        fn new() -> SetOfStacks {
            let first_stack = Stack { stack: Vec::new() };
            SetOfStacks {
                stacks: vec![first_stack],
                limit: 5,
            }
        }

        fn display(&self) {
            for stack in &self.stacks {
                for v in &stack.stack {
                    print!("{} -> ", v);
                }
                print!(" | ");
            }
            print!("None\n");
        }

        fn get_last_stack(&mut self) -> &mut Stack {
            let last_stack_index = self.stacks.len() - 1;
            self.stacks.get_mut(last_stack_index).unwrap()
        }

        fn add_stack(&mut self) {
            self.stacks.push(Stack {
                stack: Vec::new(),
            });
        }

        fn push(&mut self, value: i32) {
            if self.stacks.is_empty() {
                self.add_stack();
            }

            let limit = self.limit;
            let mut current_stack = self.get_last_stack();
            if current_stack.stack.len() >= limit as usize {
                self.add_stack();
                current_stack = self.get_last_stack();
            }

            current_stack.stack.push(value);
        }

        fn pop(&mut self) {
            let current_vec = self.get_last_stack();
            if current_vec.stack.is_empty() { return; }

            if current_vec.stack.len() == 1 {
                self.stacks.pop();
            } else {
                current_vec.stack.pop();
            }


        }

        fn pop_at(&mut self, stack_index: usize) -> Option<i32> {
            if self.stacks.is_empty() {
                return None;
            }

            let value = self.stacks[stack_index].stack.pop();
            let mut temporal_value = None;
            for i in (stack_index..self.stacks.len()).rev() {
                let current_stack = self.stacks.get_mut(i)?;
               
                if let Some(tmp) = temporal_value {
                    current_stack.stack.push(tmp)
                }

                if i != stack_index {
                    temporal_value = Some(current_stack.stack.remove(0));
                }
            }

            if self.stacks.last()?.stack.is_empty() && self.stacks.len() > 1 {
                self.stacks.pop();
            }

            value
        }
    }

    let mut stacks = SetOfStacks::new();
    stacks.pop();
    for i in 0..12 {
        stacks.push(i);
    }
    stacks.display();
    for _ in 0..4 {
        stacks.pop();
    }
    stacks.display();

    for i in 8..16 {
        stacks.push(i);
    }
    stacks.display();
    stacks.pop_at(1);
    stacks.display();

}