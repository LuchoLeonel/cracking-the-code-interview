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
        head: TreeNode,
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
            new_node.borrow_mut().left = Node::new(left, Some(Rc::clone(&new_node)));
            new_node.borrow_mut().right = Node::new(right, Some(Rc::clone(&new_node)));

            Some(new_node)
        }
        fn search_min_child_on_left(node: &TreeNode) -> TreeNode {
            let mut current = node.clone();
            while let Some(next) = current.as_ref().map(|n| n.borrow().left.clone()).flatten() {
                current = Some(next);
            }
            current
        }
        fn search_upper(tree_node: &TreeNode) -> (TreeNode, TreeNode) {
            let Some(node) = tree_node else {
                return (None, None);
            };
            
            let mut current_child = node.clone();
            let mut current_father = node.borrow().father.clone();
            while let Some(father) = current_father {
                if let Some(right) = &father.clone().borrow().right {
                    if !Rc::ptr_eq(&current_child, right) {
                        return (Some(current_child), Some(father));
                    }
                }
                
                current_child = father.clone();
                current_father = father.borrow().father.clone();
            }
            (Some(current_child.clone()), Some(current_child.clone()))
        }
    }

    impl Tree {
        fn new(list: &[i32]) -> Tree {
            Tree { head: Node::new(list, None) }
        }
        fn find_next_successor(&self, node: TreeNode) -> TreeNode {
            let Some(ref current_node) = node else {
                return None;
            };

            let maybe_right_node_min = Node::search_min_child_on_left(&current_node.borrow().right.clone());
            if maybe_right_node_min.is_some() {
                return maybe_right_node_min;
            }

            if let (Some(previous), Some(upper)) = Node::search_upper(&node) {
                if let Some(left) = &upper.clone().borrow().left {
                    if Rc::ptr_eq(&previous, &left) {
                        return Some(upper);
                    }
                }
                return Node::search_min_child_on_left(&upper.borrow().right.clone());
            }

            None
        }

    }

    let list = vec![1, 5, 10, 15, 20, 25, 30, 35, 40, 50, 60, 70, 80, 90, 100];
    let tree = Tree::new(&list);
    let one_node = tree.head.clone()
        .unwrap().borrow().left.clone()
        .unwrap().borrow().left.clone()
        .unwrap().borrow().left.clone();

    print!("{}\n", tree.find_next_successor(one_node).unwrap().borrow().value);
    
}