use std::rc::Rc;
use std::cell::RefCell;

pub fn run() {
    type Link = Option<Rc<RefCell<Node>>>;

    #[derive(Debug, Clone, PartialEq)]
    struct Node {
        value: String,
        position: i32,
        next: Link,
        previous: Link,
    }

    #[derive(Debug, Clone)]
    struct LinkedList {
        head: Link,
        tail: Link,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum AnimalKind {
        Dog,
        Cat,
    }

    #[derive(Debug)]
    struct AnimalShelter {
        dogs: LinkedList,
        cats: LinkedList,
        counter: i32,
    }

    impl AnimalKind {
        fn display(&self) -> &str {
            match self {
                AnimalKind::Dog => "Dog",
                AnimalKind::Cat => "Cat",
            }
        }
    }


    impl LinkedList {
        fn new() -> LinkedList {
            LinkedList { head: None, tail: None }
        }

        fn display(&self) {
            let mut current = self.head.clone();
            while let Some(node) = current {
                print!("{} ({}) -> ", node.borrow().value, node.borrow().position);
                current = node.borrow().next.clone();
            }
            print!("None\n");
        }

        fn enqueue(&mut self, name: &str, position: i32) {
            let new_node = Rc::new(RefCell::new(Node {
                value: name.to_string(),
                position,
                next: self.head.clone(),
                previous: None,
            }));

            if let Some(old_head) = self.head.take() {
                old_head.borrow_mut().previous = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
            } else {
                self.tail = Some(new_node.clone());
            }

            self.head = Some(new_node);
        }

        fn dequeue(&mut self) -> Link {
            self.tail.take().map(|old_tail| {
                if let Some(prev) = old_tail.clone().borrow().previous.clone() {
                    prev.borrow_mut().next.take();
                    self.tail = Some(prev);
                } else {
                    self.head.take();
                }
                old_tail
            })
        }
    }

    impl AnimalShelter {
        fn new() -> AnimalShelter {
            AnimalShelter {
                cats: LinkedList::new(),
                dogs: LinkedList::new(),
                counter: 0,
            }
        }
        fn display(&self) {
            for v in vec![self.dogs.clone(), self.cats.clone()] {
                v.display();
            }
            print!("\n");
        }

        fn enqueue(&mut self, name: &str, kind: AnimalKind) {
            let position = self.counter;

            match kind {
                AnimalKind::Dog => self.dogs.enqueue(name, position),
                AnimalKind::Cat => self.cats.enqueue(name, position),
            };
            self.counter+=1;
        }
        fn dequeue_dog(&mut self) -> Link {
            self.dogs.dequeue()
        }
        fn dequeue_cat(&mut self) -> Link {
            self.cats.dequeue()
        }
        fn dequeue_any(&mut self) -> Link {
            let dogs_tail = self.dogs.tail.clone();
            let cats_tail = self.cats.tail.clone();
            match (dogs_tail, cats_tail) {
                (Some(dog), Some(cat)) => {
                    if dog.borrow().position < cat.borrow().position {
                        self.dequeue_dog()
                    } else {
                        self.dequeue_cat()
                    }
                },
                (Some(_dog), None) => self.dequeue_dog(),
                (None, Some(_cat)) => self.dequeue_cat(),
                (None, None) => None,
            }
        }

    }

    let mut animal_shelter = AnimalShelter::new(); 
    animal_shelter.enqueue("Chino", AnimalKind::Cat);
    animal_shelter.enqueue("Lio", AnimalKind::Cat);
    animal_shelter.enqueue("Manchitas", AnimalKind::Dog);
    animal_shelter.enqueue("Puchi", AnimalKind::Dog);
    animal_shelter.enqueue("Joni", AnimalKind::Dog);
    animal_shelter.enqueue("Levis", AnimalKind::Cat);
    animal_shelter.enqueue("Tover", AnimalKind::Cat);
    animal_shelter.enqueue("Nilo", AnimalKind::Cat);
    animal_shelter.display();

    
    animal_shelter.dequeue_any();
    animal_shelter.display();
    
    animal_shelter.dequeue_dog();
    animal_shelter.display();

    animal_shelter.dequeue_cat();
    animal_shelter.display();

    animal_shelter.dequeue_any();
    animal_shelter.display();

    animal_shelter.dequeue_any();
    animal_shelter.display();
}