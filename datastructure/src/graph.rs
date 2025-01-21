use std::collections::HashMap;
use std::collections::HashSet;

struct Graph {
    adjacency_list: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: &str) {
        self.adjacency_list
            .entry(node.to_string())
            .or_insert(HashSet::new());
    }

    fn add_edge(&mut self, node1: &str, node2: &str) {
        self.adjacency_list
            .entry(node1.to_string())
            .or_insert(HashSet::new())
            .insert(node2.to_string());
        self.adjacency_list
            .entry(node2.to_string())
            .or_insert(HashSet::new())
            .insert(node1.to_string());
    }

    fn display(&self) {
        for (node, edges) in &self.adjacency_list {
            println!("{}: {:?}", node, edges);
        }
    }
}

fn main() {
    let mut graph = Graph::new();
    graph.add_node("A");
    graph.add_node("B");
    graph.add_node("C");
    graph.add_edge("A", "B");
    graph.add_edge("B", "C");
    graph.display();
}
