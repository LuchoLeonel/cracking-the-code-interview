use std::rc::Rc;
use std::cell::RefCell;

pub fn run() {
    type Link = Option<Rc<RefCell<Node>>>;

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
            let new_node = Rc::new(RefCell::new(Node {
                value,
                next: self.head.take(),
            }));

            self.head = Some(new_node);
        }

        fn display(&self) {
            let mut current = self.head.clone();
            for _ in 0..10 {
                if let Some(node) = current {
                    print!("{} -> ", node.borrow().value);
                    current = node.borrow().next.clone();
                } else {
                    print!("None\n");
                    return;
                }
            }
            print!("...\n");
        }
        fn is_corrupt(&self) -> bool {
            let mut nodes: Vec<Rc<RefCell<Node>>> = Vec::new();
            let mut current = self.head.clone();
            while let Some(node) = current {
                for n in &nodes {
                    if Rc::ptr_eq(&n, &node) {
                        return true;
                    }
                }
                
                nodes.push(node.clone());
                current = node.borrow().next.clone();
            }
            false
        }
    }

    let mut list1 = LinkedList::new();
    list1.append(2);
    list1.append(5);
    list1.append(8);
    list1.display();
    print!("{}\n", list1.is_corrupt());


    let mut list2 = LinkedList::new();
    list2.append(6);
    list2.append(8);
    list2.append(3);
    list2.append(2);
    list2.append(3);
    list2.append(1);
    let mut current = list2.head.clone();
    let mut counter = 0;
    let mut node_ref = None;
    while let Some(node) = current {
        if counter == 0 {
            node_ref = Some(node.clone());
        }
        if counter == 5 {
            node.borrow_mut().next = node_ref.clone();
            break;
        }
        counter+=1;
        current = node.borrow().next.clone();
    }
    list2.display();
    print!("{}\n", list2.is_corrupt());
}