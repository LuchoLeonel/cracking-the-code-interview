use std::rc::Rc;
use std::cell::RefCell;

pub fn run() {
    type TreeNode = Option<Rc<RefCell<Node>>>;

    #[derive(Debug)]
    struct Node {
        value: i32,
        next_1: TreeNode,
        next_2: TreeNode,
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
            let left = &list[0..middle];
            let right = &list[middle+1..];
            Some(Rc::new(RefCell::new(Node {
                value: list[middle],
                next_1: Node::new(left),
                next_2: Node::new(right),
            })))
        }
    }

    impl Tree {
        fn new(list: &[i32]) -> Tree {
            Tree {
                head: Node::new(list)
            }
        }
    }


    let list = vec![1, 5, 10, 15, 20, 25, 30];
    let _tree = Tree::new(&list);


    let list = vec![5, 10, 15, 20, 25, 30];
    let _tree = Tree::new(&list);
}