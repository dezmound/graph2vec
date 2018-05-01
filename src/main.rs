mod graph;

use std::fs::File;
use std::io::Read;
use graph::to_vec::ToVec;
use std::sync::Arc;
use graph::vectorize::BoW;
use graph::vectorize::BoWVertex;

fn main() {
    let mut f = File::open("out.json").expect("Open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let mut g = graph::Graph::from_json(& contents);
    g.set_root("1");
    let vectorize = Arc::new(BoWVertex::new(Arc::new(g.clone())));
    println!("{:?}", g.set_vectorize(vectorize).to_vec());
}