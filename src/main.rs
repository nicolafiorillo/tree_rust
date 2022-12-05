use std::fs::File;
use std::io::{Error, Write};

mod tree;
use tree::*;

mod graph_viz;
use graph_viz::GraphViz;

fn main() {
    let mut tree = Tree::Empty;
    tree.add(1);
    tree.add(3);
    tree.add(2);
    tree.add(5);
    tree.add(4);

    let mut graph = GraphViz::new();
    graph.read_tree(&tree);
    let content = graph.write();

    let file_name = "tree.dot";
    match save_to_file(file_name, &content) {
        Ok(_) => println!("Tree image {} created.", file_name),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn save_to_file(file_name: &str, content: &str) -> Result<(), Error> {
    let mut output = File::create(file_name)?;
    write!(output, "{}", content)?;

    Ok(())
}
