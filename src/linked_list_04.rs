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
            let mut vec = Vec::new();
            for ch in sum.to_string().chars() {
                let number = ch.to_string().parse::<i32>().unwrap();
                vec.push(number);
            }

            let mut new_list = LinkedList { head: None };
            for i in vec {
                new_list.append(i);
            }

            new_list
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
            let mut numbers = Vec::new();
            let mut current = &self.head;
            while let Some(node) = current {
                numbers.push(node.value);
                current = &node.next;
            }

            if numbers.is_empty() {
                return None;
            }
            
            let mut string = String::new();
            let len = numbers.len();
            let mut i = len-1;
            for _ in 0..len {
                string = format!("{}{}", string, numbers[i]);
                if i > 0 {
                    i-=1;
                }
            }

            string.parse::<i32>().ok()
        }

        fn sum_list(&mut self) {

        }
    }


    let mut list1 = LinkedList::new();
    list1.append(6);
    list1.append(1);
    list1.append(7);
    list1.display();
    let number1 = list1.convert_to_number().unwrap();
    print!("{}\n", number1);
    let mut list2 = LinkedList::new();
    list2.append(2);
    list2.append(9);
    list2.append(5);
    list2.display();
    let number2 = list2.convert_to_number().unwrap();
    print!("{}\n", number2);


    let list3 = LinkedList::from_sum(number1, number2);
    list3.display();
    let number3 = list3.convert_to_number().unwrap();
    print!("{}\n", number3);
}
