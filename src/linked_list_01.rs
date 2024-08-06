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
            print!("None\n")
        }

        fn nth_to_last(&self, index: i32) -> Option<i32> {
            let mut linked_list_len = 0;
            let mut current = &self.head;

            while let Some(node) = current {
                linked_list_len+=1;
                current = &node.next;
            }

            if linked_list_len <= index {
                return None;
            }


            current = &self.head;
            let mut counter = 0;
            let mut value = 0;
            while let Some(node) = current {
                counter+=1;
                if linked_list_len - index == counter {
                    value = node.value;
                }
                current = &node.next;
            }

            Some(value)
        }

        fn nth_to_last_chat_gpt(&self, index: i32) -> Option<i32> {
            if index < 0 {
                return None;
            }
        
            let mut p1 = &self.head;
            let mut p2 = &self.head;
        
            // Mover p1 index pasos adelante
            for _ in 0..=index {
                if let Some(node) = p1 {
                    p1 = &node.next;
                } else {
                    return None; // Si la lista tiene menos de index+1 elementos
                }
            }
        
            // Mover ambos punteros hasta que p1 alcance el final
            while let Some(node) = p1 {
                p1 = &node.next;
                p2 = if let Some(p2_node) = p2 {
                    &p2_node.next
                } else {
                    return None;
                };
            }
        
            p2.as_ref().map(|node| node.value)
        }
    }

    fn print(index: Option<i32>) {
        if let Some(i) = index {
            print!("{}\n", i);
        } else {
            print!("None\n");
        }
    }


    let mut list = LinkedList::new();
    list.append(1);
    list.append(4);
    list.append(6);
    list.append(23);
    list.append(2);
    list.append(75);
    list.display();
    print(list.nth_to_last(0));
    print(list.nth_to_last(1));
    print(list.nth_to_last(2));
    print(list.nth_to_last(3));
    print(list.nth_to_last(4));
    print(list.nth_to_last(5));
    print(list.nth_to_last(6));
    print(list.nth_to_last(55));
}

