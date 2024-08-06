pub fn run() {

    #[derive(Debug)]
    struct ThreeStacksArray {
        vector: Vec<i32>,
    }

    impl ThreeStacksArray {
        fn new() -> ThreeStacksArray {
            let mut vector = Vec::new();
            vector.push(2);
            vector.push(3);
            ThreeStacksArray { vector }
        }

        fn display(&self, beggining: usize, finish: usize) {
            if beggining >= finish || beggining >= self.vector.len() {
                return print!("Empty stack\n");
            }
            for i in beggining..finish {
                print!("{} -> ", self.vector[i]);
            }
            print!("Index: {}\n", finish);
        }

        fn display_stack_one(&self) {
            let beggining = 2 as usize;
            self.display(beggining, self.index_stack_one());
        }

        fn display_stack_two(&self) {
            self.display(self.index_stack_one(), self.index_stack_two());
        }

        fn display_stack_three(&self) {
            self.display(self.index_stack_two(), self.vector.len());
            print!("\n");
        }

        fn index_stack_one(&self) -> usize {
            self.vector[0] as usize
        }

        fn index_stack_two(&self) -> usize {
            self.vector[1] as usize
        }

        fn push_stack_one(&mut self, value: i32) {
            self.vector.insert(self.index_stack_one(), value);
            self.vector[0]+=1;
            self.vector[1]+=1;
        }
        fn push_stack_two(&mut self, value: i32) {
            let index = self.index_stack_two();
            if index < self.vector.len() {
                self.vector.insert(index, value);
                self.vector[1]+=1;
            } else {
                self.vector.insert(index-1, value);
            }
        }
        fn push_stack_three(&mut self, value: i32) {
            self.vector.push(value);
        }


        fn pop_stack_one(&mut self) {
            if self.index_stack_one() == 2 {
                return print!("Cannot remove item because stack is empty\n");
            }
            self.vector.remove(self.index_stack_one());
            self.vector[0]-=1;
            self.vector[1]-=1;
        }
        fn pop_stack_two(&mut self) {
            if self.index_stack_one() == (self.index_stack_two() -1) {
                return print!("Cannot remove item because stack is empty\n");
            }

            self.vector.remove(self.index_stack_two());
            self.vector[1]-=1;
        }
        fn pop_stack_three(&mut self) {
            if self.index_stack_two() == (self.vector.len()-1) {
                return print!("Cannot remove item because stack is empty\n");
            }
            self.vector.pop();
        }
    }

    let mut three_stacks = ThreeStacksArray::new();
    assert_eq!(three_stacks.vector[0], 2);
    assert_eq!(three_stacks.vector[1], 3);


    three_stacks.display_stack_one();
    three_stacks.display_stack_two();
    three_stacks.display_stack_three();
    
    three_stacks.push_stack_one(2);
    three_stacks.push_stack_two(4);
    three_stacks.push_stack_three(3);

    three_stacks.display_stack_one();
    three_stacks.display_stack_two();
    three_stacks.display_stack_three();


    three_stacks.push_stack_one(7);
    three_stacks.push_stack_two(3);
    three_stacks.push_stack_three(8);

    three_stacks.display_stack_one();
    three_stacks.display_stack_two();
    three_stacks.display_stack_three();

    
    three_stacks.push_stack_one(44);
    three_stacks.push_stack_three(88);


    three_stacks.display_stack_one();
    three_stacks.display_stack_two();
    three_stacks.display_stack_three();
    
    three_stacks.pop_stack_one();
    three_stacks.pop_stack_two();
    three_stacks.pop_stack_three();

    three_stacks.display_stack_one();
    three_stacks.display_stack_two();
    three_stacks.display_stack_three();


    three_stacks.pop_stack_one();
    three_stacks.pop_stack_two();
    three_stacks.pop_stack_three();

    three_stacks.display_stack_one();
    three_stacks.display_stack_two();
    three_stacks.display_stack_three();

    three_stacks.pop_stack_one();
    three_stacks.pop_stack_two();
    three_stacks.pop_stack_three();


    three_stacks.display_stack_one();
    three_stacks.display_stack_two();
    three_stacks.display_stack_three();


    three_stacks.pop_stack_one();
    three_stacks.pop_stack_two();
    three_stacks.pop_stack_three();

    assert_eq!(three_stacks.vector[0], 2);
    assert_eq!(three_stacks.vector[1], 3);
}