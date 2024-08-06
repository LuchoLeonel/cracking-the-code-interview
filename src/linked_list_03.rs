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

        fn partition(&mut self, number: i32) {

            let mut left = LinkedList::new();
            let mut right = LinkedList::new();

            let mut current = &self.head;
            while let Some(node) = current {
                if node.value < number {
                    left.append(node.value);
                } else {
                    right.append(node.value);
                }
                current = &node.next;
            }

            let mut final_list = LinkedList::new();
            current = &right.head;
            while let Some(node) = current {
                final_list.append(node.value);
                current = &node.next;
            }
            
            current = &left.head;
            while let Some(node) = current {
                final_list.append(node.value);
                current = &node.next;
            }

            *self = final_list;
        }
    }


    let mut list = LinkedList::new();
    list.append(1);
    list.append(2);
    list.append(10);
    list.append(5);
    list.append(8);
    list.append(5);
    list.append(3);
    list.display();
    list.partition(5);
    list.display();
}