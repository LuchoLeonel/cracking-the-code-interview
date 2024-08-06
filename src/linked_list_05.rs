pub fn run() {
    type Link = Option<Box<Node>>;

    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Link,
    }

    #[derive(Debug)]
    struct LinkedList {
        head: Link,
    }

    impl LinkedList {
        fn new() -> LinkedList {
            LinkedList { head: None }
        }

        fn from_sum(number1: i32, number2: i32) -> LinkedList {
            let sum = number1 + number2;
            let mut list = LinkedList { head: None };
            let mut chars: Vec<char> = sum.to_string().chars().collect();
            for _ in 0..chars.len() {
                let n = chars.pop().unwrap().to_string().parse::<i32>().unwrap();
                list.append(n);
            }
            
            list
        }

        fn append(&mut self, value: i32) {
            let new_node = Box::new(Node {
                value,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        }

        fn display(&self) {
            let mut current = &self.head;
            while let Some(node) = current {
                print!("{} -> ", node.value);
                current = &node.next;
            }
            print!("None\n");
        }
        
        fn convert_to_number(&self) -> Option<i32> {
            if self.head.is_none() {
                return None;
            }

            let mut numbers = Vec::new();
            let mut current = &self.head;
            while let Some(node) = current {
                numbers.push(node.value);
                current = &node.next;
            }

            let mut string = String::new();
            for n in numbers {
                string = format!("{}{}", string, n);
            }

            string.parse::<i32>().ok()
        }
    }

    let mut list1 = LinkedList::new();
    list1.append(7);
    list1.append(1);
    list1.append(6);
    list1.display();
    let number1 = list1.convert_to_number().unwrap();
    print!("{}\n", number1);

    let mut list2 = LinkedList::new();
    list2.append(5);
    list2.append(9);
    list2.append(2);
    list2.display();
    let number2 = list2.convert_to_number().unwrap();
    print!("{}\n", number2);

    let list3 = LinkedList::from_sum(number1, number2);
    list3.display();
    let number3 = list3.convert_to_number().unwrap();
    print!("{}\n", number3);
}