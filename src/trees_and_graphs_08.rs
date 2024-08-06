use std::rc::Rc;
use std::cell::RefCell;

pub fn run() {
    type TreeNode = Option<Rc<RefCell<Node>>>;

    #[derive(Debug)]
    struct Node {
        value: i32,
        left: TreeNode,
        right: TreeNode,
    }

    #[derive(Debug)]
    struct Tree {
        head: TreeNode,
    }

    impl Node {
        fn new(value: i32) -> TreeNode {
            Some(Rc::new(RefCell::new(Node {
                value,
                left: None,
                right: None,
            })))
        }
        fn insert(node: &Rc<RefCell<Node>>, value: i32) {
            let mut node_borrow = node.borrow_mut();
            if value < node_borrow.value {
                match &node_borrow.left {
                    Some(ref node) => Node::insert(node, value),
                    None => node_borrow.left = Node::new(value),
                }
            } else {
                match &node_borrow.right {
                    Some(ref node) => Node::insert(node, value),
                    None => node_borrow.right = Node::new(value),
                }
            }
        }

    }

    impl Tree {
        fn new() -> Tree {
            Tree { head: None }
        }
        fn insert(&mut self, value: i32) {
            match self.head {
                Some(ref head) => Node::insert(head, value),
                None => self.head = Node::new(value),
            }
        }
      
    }

    let list = vec![50, 76, 32, 47, 87, 124, 149, 175, 2, 40, 34, 57, 83, 90, 73];
    let mut tree = Tree::new();
    for &value in &list {
        tree.insert(value);
    }


    dbg!(&tree);


}