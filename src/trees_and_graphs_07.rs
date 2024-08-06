use std::rc::Rc;
use std::cell::RefCell;

pub fn run() {
    type TreeNode = Option<Rc<RefCell<Node>>>;

    #[derive(Debug)]
    struct Node {
        value: i32,
        father: TreeNode,
        left: TreeNode,
        right: TreeNode,
    }

    #[derive(Debug)]
    struct Tree {
        head: TreeNode
    }

    impl Node {
        fn new(list: &[i32], father: TreeNode) -> TreeNode {
            if list.is_empty() {
                return None;
            }
            let middle = list.len() / 2;
            let left = &list[..middle];
            let right = &list[middle+1..];

            let new_node = Rc::new(RefCell::new(Node {
                value: list[middle],
                father,
                left: None,
                right: None,
            }));

            let node = Some(new_node.clone());
            new_node.borrow_mut().left = Node::new(left, node.clone());
            new_node.borrow_mut().right = Node::new(right, node.clone());
            node
        }
    }

    impl Tree {
        fn new(list: &[i32]) -> Tree {
            Tree {
                head: Node::new(list, None),
            }
        }

        fn find_common_ancestor(&self, tree_node_1: TreeNode, tree_node_2: TreeNode) -> TreeNode {
            let (Some(node_1), Some(node_2)) = (tree_node_1, tree_node_2) else {
                return None;
            };

            let mut current_1 = node_1.borrow().father.clone();
            while let Some(father_1) = current_1 {
                let mut current_2 = node_2.borrow().father.clone();
                while let Some(father_2) = current_2 {
                    if Rc::ptr_eq(&father_1, &father_2) {
                        return Some(father_1.clone());
                    }
                    current_2 = father_2.borrow().father.clone();
                }
                current_1 = father_1.borrow().father.clone();
            }

            None
        }
    }


    let list = vec![1, 5, 10, 15, 20, 25, 30, 35, 40, 50, 60, 70, 80, 90, 100];
    let tree = Tree::new(&list);

    let node_1 = tree.head.clone().unwrap()
        .borrow().left.clone().unwrap()
        .borrow().left.clone().unwrap()
        .borrow().left.clone();
    
    let node_2 = tree.head.clone().unwrap()
        .borrow().right.clone().unwrap()
        .borrow().left.clone();


    let ancestor = tree.find_common_ancestor(node_1, node_2);
    print!("Ancestor: {}\n", ancestor.unwrap().borrow().value);
}