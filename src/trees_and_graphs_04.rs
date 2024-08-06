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
        fn new(list: &[i32]) -> TreeNode {
            if list.is_empty() {
                return None;
            }

            let middle = list.len() / 2;
            let left = &list[..middle];
            let right = &list[middle+1..];

            Some(Rc::new(RefCell::new(Node {
                value: list[middle],
                left: Node::new(left),
                right: Node::new(right),
            })))
        }
    }

    #[derive(Debug, PartialEq)]
    enum Direction {
        Left,
        Right,
    }

    impl Tree {
        fn new(list: &[i32]) -> Tree {
            Tree {
                head: Node::new(list)
            }
        }
        fn is_binary_search(&self) -> bool {
            fn check_binary_search(node_tree: &TreeNode, father: Option<(i32, Direction)>) -> Result<Option<i32>, ()> {
                let Some(node) = node_tree else {
                    return Ok(None);
                };

                let n = node.borrow();

                if let Some((father_value, direction)) = father {
                    let error_left = direction == Direction::Left && n.value > father_value;
                    let error_right = direction == Direction::Right && n.value < father_value;
                    if error_left || error_right  {
                        return Err(());
                    }
                }

                let left = check_binary_search(&n.left, Some((n.value, Direction::Left)))?;
                let right = check_binary_search(&n.right, Some((n.value, Direction::Right)))?;

                if left.map_or(false, |l| l > n.value) {
                    return Err(());
                }
                if right.map_or(false, |r| r < n.value) {
                    return Err(());
                }

                Ok(Some(n.value))
            }

            check_binary_search(&self.head, None).is_ok()
        }
    }



    let list = vec![1, 5, 10, 15, 20, 25, 30];
    let tree = Tree::new(&list);
    print!("Is binary search?: {}\n", tree.is_binary_search());
}