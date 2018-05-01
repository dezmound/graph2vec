pub mod to_vec;
pub mod vectorize;
extern crate json;

use std::sync::Arc;
use std::iter::Iterator;
use std::collections::LinkedList as Stack;
use graph::vectorize::Vectorize;

#[derive(Debug, Clone)]
pub struct Graph {
    vectorize: Vectorize,
    visit_stack: Stack<Arc<Node>>,
    iter_stack: Stack<Arc<Node>>,
    pub root: Option<Arc<Node>>,
    pub nodes: Vec<Arc<Node>>,
    pub edges: Vec<Edge>
}
#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub label: String
}
#[derive(Debug, Clone)]
pub struct Edge {
    pub source: Arc<Node>,
    pub target: Arc<Node>
}

pub struct GraphIterator {
    graph: Graph
}

impl Graph {
    pub fn from_json(json: & String) -> Graph {
        if let json::JsonValue::Object(deserialized) = json::parse(json.as_ref()).unwrap() {
            let nodes : Vec<Arc<Node>> = deserialized.get("nodes").unwrap().members()
                .map(|v| {
                    if let json::JsonValue::Object(ref val) = *v {
                        return Arc::new(Node {
                            id: val.get("id").unwrap().to_string(),
                            label: val.get("label").unwrap().to_string()
                        });
                    }
                    panic!("Invalid structure of json graph body.")
            }).collect::<Vec<Arc<Node>>>();
            let edges : Vec<Edge> = deserialized.get("edges").unwrap().members()
                .map(|v| {
                    if let json::JsonValue::Object(ref val) = *v {
                        let source = nodes.iter().find(|&v| v.id ==  val.get("source").unwrap().to_string()).unwrap();
                        let target = nodes.iter().find(|&v| v.id ==  val.get("target").unwrap().to_string()).unwrap();
                        if source.id == target.id {
                            panic!("Selflooped node detected!");
                        }
                        return Edge {
                            source: Arc::clone(&source),
                            target: Arc::clone(&target)
                        };
                    }
                    panic!("Invalid structure of json graph body.")
                }).collect::<Vec<Edge>>();
            return Graph {
                nodes,
                edges,
                root: None,
                iter_stack: Stack::new(),
                visit_stack: Stack::new(),
                vectorize: Vectorize::None
            }
        }
        panic!("Incorrect struct of json contains!");
    }
    pub fn set_root(&mut self, id: &str) -> bool {
        match self.nodes.iter().find(|&v| v.id == id) {
            Some(node) => {
                self.root = Some(Arc::clone(node));
                self.iter_stack.push_back(Arc::clone(node));
                return true;
            },
            None => false
        }
    }
    pub fn set_vectorize(&mut self, vectorize: Arc<to_vec::ToVec>) -> &mut Self {
        self.vectorize = Vectorize::Algo(vectorize);
        self
    }
    pub fn iter(&self) -> GraphIterator {
        GraphIterator {
            graph: self.clone()
        }
    }
}

impl Iterator for GraphIterator {
    type Item = Arc<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_node) = self.graph.iter_stack.pop_back() {
            let mut filtered = self.graph.edges.iter().filter(
                |&e| {
                    if e.source.id == current_node.id || e.target.id == current_node.id {
                        if let None = self.graph.visit_stack.iter().find(|&v| {
                            if e.target.id != current_node.id {
                                return v.id == e.target.id;
                            }
                            return v.id == e.source.id;
                        }) {
                            return true;
                        }
                    }
                    false
            }).map(|v| {
                if v.source.id == current_node.id {
                    return Arc::clone(&v.target);
                }
                Arc::clone(&v.source)
            }).collect::<Stack<Arc<Node>>>();
            self.graph.iter_stack.append(&mut filtered);
            self.graph.visit_stack.push_front(Arc::clone(&current_node));
            return Some(current_node);
        }
        None
    }
}

impl to_vec::ToVec for Graph {
    fn to_vec(&self) -> Vec<f32> {
        match self.vectorize {
            Vectorize::None => panic!("Algorithm for vectorization are not selected!"),
            Vectorize::Algo(ref algo) => {
                algo.to_vec()
            }
        }
    }
}