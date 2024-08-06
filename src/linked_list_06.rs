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

        fn is_palindrome(&self) -> bool {
            let mut numbers1: Vec<i32> = Vec::new();
            
            let mut current = &self.head;
            while let Some(node) = current {
                numbers1.push(node.value);
                current = &node.next;
            }

            let numbers2: Vec<i32> = numbers1.iter().rev().map(|n| n.clone()).collect();
            for i in 0..numbers1.len() {
                if numbers1[i] != numbers2[i] {
                    return false;
                }
            }

            true
        }
    }

    let mut list1 = LinkedList::new();
    list1.append(1);
    list1.append(2);
    list1.append(3);
    list1.append(2);
    list1.append(1);
    list1.display();
    print!("{}\n", list1.is_palindrome());

    let mut list2 = LinkedList::new();
    list2.append(1);
    list2.append(2);
    list2.append(3);
    list2.append(4);
    list2.append(5);
    list2.display();
    print!("{}\n", list2.is_palindrome());
}