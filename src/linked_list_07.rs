use std::rc::Rc;
use std::cell::RefCell;

pub fn run() {
    type Link = Option<Rc<RefCell<Node>>>;

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
            let new_node = Rc::new(RefCell::new(Node{
                value,
                next: self.head.take(),
            }));

            self.head = Some(new_node);
        }

        fn display(&self) {
            let mut current = self.head.clone();
            while let Some(node) = current {
                print!("{} -> ", node.borrow().value);
                current = node.borrow().next.clone();
            }
            print!("None\n");
        }

        fn is_intersected_by(&self, list: LinkedList) -> bool {
            
            let mut current1 = self.head.clone();
            while let Some(node1) = current1 {
                let mut current2 = list.head.clone();
                while let Some(node2) = current2 {
                    if Rc::ptr_eq(&node1, &node2) {
                        return true;
                    }
                    current2 = node2.borrow().next.clone();
                }
                current1 = node1.borrow().next.clone();
            }
            
            false
        }
    }

    let mut list1 = LinkedList::new();
    list1.append(1);
    list1.append(2);
    list1.append(3);
    list1.append(4);
    list1.append(5);
    list1.append(6);
    list1.append(7);
    list1.display();

    let mut list2 = LinkedList::new();
    
    // Crear una verdadera intersección
    let mut current = list1.head.clone();
    let mut counter = 0;
    while let Some(node) = current {
        if counter == 3 {
            list2.head = Some(Rc::clone(&node));
            break;
        }
        counter += 1;
        current = node.borrow().next.clone();
    }

    list2.append(2);
    list2.append(5);
    list2.display();

    let result = list1.is_intersected_by(list2);
    print!("{}\n", result);

    let mut list3 = LinkedList::new();
    list3.append(3);
    list3.append(5);
    list3.append(8);
    list3.append(2);
    list3.display();

    let result = list1.is_intersected_by(list3);
    print!("{}\n", result);


}