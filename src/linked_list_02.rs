pub fn run() {
    type Link = Option<Box<Node>>;

    #[derive(Debug, Clone)]
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
            let new_node = Box::new(Node{
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

        fn delete_middle(&mut self) {
            let third_node = self.head.clone().map(|n| n.next).flatten().map(|n| n.next).flatten();
            if third_node.is_none() {
                return;
            }

            let mut number_of_nodes = 0;
            let mut current = &self.head;
            while let Some(node) = current {
                number_of_nodes+=1;
                current = &node.next;
            }

            let mut current_mut = self.head.as_mut();
            let middle = number_of_nodes / 2;
            let mut i = 0;
            while let Some(node) = current_mut {
                if i == middle {
                    let next_node = node.next.as_mut().unwrap();
                    node.value = next_node.value;
                    node.next = next_node.next.take();
                    break;
                }
                i+=1;
                current_mut = node.next.as_mut();
            }
        }
    }

    let mut list1 = LinkedList::new();
    list1.append(4);
    list1.append(7);
    list1.append(55);
    list1.append(31);
    list1.append(77);
    list1.append(84);
    list1.display();
    list1.delete_middle();
    list1.display();
    list1.delete_middle();
    list1.display();
    list1.delete_middle();
    list1.display();
    list1.delete_middle();
    list1.display();
    list1.delete_middle();
    list1.display();

}