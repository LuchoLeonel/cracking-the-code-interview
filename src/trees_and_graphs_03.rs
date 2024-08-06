use std::rc::Rc;
use std::cell::RefCell;

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
        head: TreeNode
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
        fn is_balanced(&self) -> bool {
            fn check_balanced(tree_node: &TreeNode) -> Result<i32, ()> {
                let Some(node) = tree_node.clone() else {
                    return Ok(0);
                };

                let n = node.borrow();
                let left_heigh = check_balanced(&n.left_next)?;
                let right_heigh = check_balanced(&n.right_next)?;
                if (left_heigh - right_heigh).abs() < 2 {
                    Ok(1 + left_heigh.max(right_heigh))
                } else {
                    Err(())
                }
            }

            check_balanced(&self.head).is_ok()
        }
    }

    fn print_balanced(balanced: bool) {
        print!("Balanced?: {}\n", balanced);
    }

    let list = vec![1, 5, 10, 15, 20, 25, 30];
    let tree = Tree::new(&list);
    print_balanced(tree.is_balanced())
}