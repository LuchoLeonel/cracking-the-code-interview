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

        fn bst_sequences(node: TreeNode) -> Vec<Vec<i32>> {
            if node.is_none() {
                return vec![vec![]];
            }

            let node = node.unwrap();
            let node_borrow = node.borrow();

            let mut result = Vec::new();
            let prefix = vec![node_borrow.value];

            let left_seq = Node::bst_sequences(node_borrow.left.clone());
            let right_seq = Node::bst_sequences(node_borrow.right.clone());

            for left in left_seq {
                for right in right_seq.iter() {
                    let mut weaved = Vec::new();
                    Node::weave_lists(&left, right, &mut weaved, prefix.clone());
                    result.extend(weaved);
                }
            }

            result
        }

        fn weave_lists(first: &Vec<i32>, second: &Vec<i32>, results: &mut Vec<Vec<i32>>, prefix: Vec<i32>) {
            if first.is_empty() || second.is_empty() {
                let mut result = prefix.clone();
                result.extend(first.iter());
                result.extend(second.iter());
                results.push(result);
                return;
            }

            let mut first_rest = first.clone();
            let head_first = first_rest.remove(0);
            let mut new_prefix = prefix.clone();
            new_prefix.push(head_first);
            Node::weave_lists(&first_rest, second, results, new_prefix);

            let mut second_rest = second.clone();
            let head_second = second_rest.remove(0);
            let mut new_prefix = prefix.clone();
            new_prefix.push(head_second);
            Node::weave_lists(first, &second_rest, results, new_prefix);
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

        fn bst_sequences(&self) -> Vec<Vec<i32>> {
            Node::bst_sequences(self.head.clone())
        }
    }

    let list = vec![50, 76, 32, 47, 87, 124];
    let mut tree = Tree::new();
    for &value in &list {
        tree.insert(value);
    }

    let sequences = tree.bst_sequences();
    for seq in sequences {
        println!("{:?}", seq);
    }
}
