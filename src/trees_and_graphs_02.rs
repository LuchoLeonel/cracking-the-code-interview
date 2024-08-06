use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

pub fn run() {
    type TreeNode = Option<Rc<RefCell<Node>>>;

    #[derive(Debug)]
    struct Node {
        value: i32,
        left_next: TreeNode,
        right_next: TreeNode,
    }

    #[derive(Debug)]
    struct Tree {
        head: TreeNode,
    }

    impl Node {
        fn new(list: &[i32]) -> TreeNode {
            if list.is_empty() {
                return None;
            }
            let middle = list.len() / 2;
            let left = &list[..middle];
            let right = &list[middle+1..]; 

            Some(Rc::new(RefCell::new(Node {
                value: list[middle],
                left_next: Node::new(&left),
                right_next: Node::new(&right),
            })))
        }
    }



    impl Tree {
        fn new(list: &[i32]) -> Tree {
            Tree {
                head: Node::new(list)
            }
        }
        fn nodes_at_depth(&self, depth: i32) -> Vec<i32> {
            let Some(head) = self.head.clone() else {
                return Vec::new();
            };
            let mut current_level = VecDeque::new();
            current_level.push_back(head);

            for _ in 0..depth {
                let mut temp = VecDeque::new();
                while let Some(node) = current_level.pop_front() {
                    let n = node.borrow();
                    let nodes = vec![n.left_next.clone(), n.right_next.clone()];
                    nodes.into_iter().filter_map(|nod| nod).for_each(|nod| temp.push_back(nod));
                }
                current_level = temp;
            }
           
            let mut numbers = Vec::new();
            for n in current_level {
                numbers.push(n.borrow().value);
            }
            numbers
        }
    }

    fn print_list(list: Vec<i32>) {
        print!("[");
        for (index, i) in list.iter().enumerate() {
            if (list.len() -1) == index {
                print!("{}", i)
            } else {
                print!("{}, ", i);
            }
        }
        print!("]\n");
    }

    let list = vec![1, 5, 10, 15, 20, 25, 30, 40, 50, 55, 60, 70, 80, 90, 100];
    let tree = Tree::new(&list);

    print_list(tree.nodes_at_depth(0));
    print_list(tree.nodes_at_depth(1));
    print_list(tree.nodes_at_depth(2));
    print_list(tree.nodes_at_depth(3));

}