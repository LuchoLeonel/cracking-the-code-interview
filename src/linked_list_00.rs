
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
    fn new() -> Self {
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

    fn remove_duplicates(&mut self) {
        let mut new_list = LinkedList::new();
        let mut current = &self.head;

        while let Some(node) = current {
            let mut search_through = &new_list.head;
            let mut found_duplicate = false;
            while let Some(n) = search_through {
                search_through = &n.next;
                if n.value == node.value {
                    found_duplicate = true;
                    break;
                }
            }

            if !found_duplicate  {
                new_list.append(node.value);
            }
            current = &node.next;
        }
        *self = new_list;
    }

}

pub fn run() {
    let mut list = LinkedList::new();
    list.append(1);
    list.append(4);
    list.append(66);
    list.append(182);
    list.append(4);
    list.display();
    list.remove_duplicates();
    list.display();
}