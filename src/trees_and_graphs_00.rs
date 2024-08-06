use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

pub fn run() {
    #[derive(Debug, PartialEq)]
    struct Node {
        value: String,
        neighbors: Vec<Rc<RefCell<Node>>>,
    }
    
    #[derive(Debug, Clone)]
    struct NodeWrapper(Rc<RefCell<Node>>);
    
    
    impl PartialEq for NodeWrapper {
        fn eq(&self, other: &NodeWrapper) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    impl Eq for NodeWrapper { }

    impl Hash for NodeWrapper {
        fn hash<H: Hasher>(&self, state: &mut H) {
            Rc::as_ptr(&self.0).hash(state);
        }
    }

    #[derive(Debug)]
    struct Graph {
        nodes: HashMap<i32, Rc<RefCell<Node>>>,
    }

    impl Graph {
        fn new() -> Graph {
            Graph { nodes: HashMap::new() }
        }
        fn add_node(&mut self, index: i32, value: &str) {
            let node = Rc::new(RefCell::new(Node {
                value: value.to_string(),
                neighbors: Vec::new(),
            }));
            self.nodes.insert(index, node);
        }
        fn get_node(&self, index: i32) -> Option<Rc<RefCell<Node>>> {
            self.nodes.get(&index).cloned()
        }
        fn add_edge(&mut self, from: i32, to: i32) {
            if let (Some(from_node), Some(to_node)) = (self.get_node(from), self.get_node(to)) {
                from_node.borrow_mut().neighbors.push(Rc::clone(&to_node));
            }
        }
        fn display_neighbors(&self, index: i32) {
            self.nodes.get(&index).map(|n| {
                print!("value: -> [ {}, ", n.borrow().value);
                n.borrow().neighbors.iter().for_each(|n| {
                    print!("{}, ", n.borrow().value);
                });
                print!("]\n")
            });
        }
        fn has_direct_route(&self, from_node: Rc<RefCell<Node>>, to_node: Rc<RefCell<Node>>) -> bool {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(NodeWrapper(Rc::clone(&from_node)));

            while let Some(current) = queue.pop_front() {
                if Rc::ptr_eq(&current.0, &to_node) {
                    return true;
                }

                for neighbor in current.0.borrow().neighbors.iter() {
                    let wrapped_neighbor = NodeWrapper(Rc::clone(neighbor));
                    if visited.insert(wrapped_neighbor.clone()) {
                        queue.push_back(wrapped_neighbor);
                    }
                }
            }
            false
        }
    }
    
    fn print(node: Option<Rc<RefCell<Node>>>) {
        print!("{}\n", node.unwrap().borrow().value);
    }

    let mut graph = Graph::new();
    graph.add_node(0, "hola");
    print(graph.get_node(0));

    graph.add_node(1, "como");
    print(graph.get_node(1));


    graph.add_node(2, "estas");
    print(graph.get_node(2));

    graph.add_edge(0, 1);
    graph.add_edge(0, 2);

    graph.display_neighbors(0);
    let node_0 = graph.get_node(0).unwrap();
    let node_1 = graph.get_node(1).unwrap();
    let node_2 = graph.get_node(2).unwrap();
    print!("{}\n", graph.has_direct_route(node_0.clone(), node_1));
    print!("{}\n", graph.has_direct_route(node_0.clone(), node_2));


    graph.add_node(3, "bien");
    let node_3 = graph.get_node(3).unwrap();
    print!("{}\n", graph.has_direct_route(node_0.clone(), node_3));
}