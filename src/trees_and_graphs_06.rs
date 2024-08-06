use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn run() {
    type GraphNode = Rc<RefCell<Project>>;

    #[derive(Debug, PartialEq)]
    struct Project {
        value: String,
        dependents: Vec<GraphNode>,
    }

    #[derive(Debug)]
    struct Graph {
        nodes: HashMap<String, GraphNode>,
    }

    impl Graph {
        fn new() -> Graph {
            Graph { nodes: HashMap::new() }
        }
        fn projects(&self) -> Vec<GraphNode> {
            self.nodes.values().cloned().collect::<Vec<GraphNode>>()
        }
        fn add_project(&mut self, value: &str) -> GraphNode {
            let v = value.to_string();
            let new_node = Rc::new(RefCell::new(Project {
                value: v.clone(),
                dependents: Vec::new(),
            }));
            self.nodes.insert(v, new_node.clone());
            new_node
        }
        fn get_project(&self, index: &str) -> Option<GraphNode> {
            self.nodes.get(index).cloned()
        }
        fn add_relation(&mut self, needed_node: &GraphNode, dependent_node: &GraphNode) {
            dependent_node.borrow_mut().dependents.push(needed_node.clone());
        }
        fn build_order(&self) -> Result<Vec<GraphNode>, ()> {
            let mut projects: VecDeque<GraphNode> = VecDeque::new();
            let mut temp: VecDeque<GraphNode> = VecDeque::from(self.projects());

            while let Some(p) = temp.pop_front() {
                let mut dependents =  p.borrow().dependents.clone();
                if dependents.is_empty() || dependents.iter().all(|n| projects.contains(n)) {
                    projects.push_back(p.clone());
                    continue;
                }
                
                while !dependents.is_empty() {
                    if dependents.iter().any(|n| Rc::ptr_eq(&p, n)) {
                        return Err(());
                    }
                    dependents = dependents.iter().flat_map(|n| n.borrow().dependents.clone()).collect();
                }

                temp.push_back(p);
                        
            }
            

            Ok(projects.into())
        }
    }

    let mut graph = Graph::new();
    let a = graph.add_project("a");
    let b = graph.add_project("b");
    let c = graph.add_project("c");
    let d = graph.add_project("d");
    let _e = graph.add_project("e");
    let f = graph.add_project("f");
    graph.add_relation(&a, &d);
    graph.add_relation(&f, &b);
    graph.add_relation(&b, &d);
    graph.add_relation(&f, &a);
    graph.add_relation(&d, &c);
    
    let ordered_list = graph.build_order().unwrap();
    for (index, n) in ordered_list.iter().enumerate() {
        if index != ordered_list.len()-1 {
            print!("{} -> ", n.borrow().value);
        } else {
            print!("{}", n.borrow().value);
        }
    }
    print!("\n");
}