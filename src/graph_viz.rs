use crate::tree::*;

#[derive(Debug, Default)]
pub struct GraphViz {
    header: String,
    footer: String,
    next_node_id: i32,
    next_edge_id: i32,
    nodes: Vec<String>,
    edges: Vec<String>,
}

impl GraphViz {
    pub fn new() -> GraphViz {
        GraphViz {
          header: String::from("digraph g { fontname=\"Helvetica,Arial,sans-serif\"; node [fontname=\"Helvetica,Arial,sans-serif\", fontsize = \"12\"]; edge [fontname=\"Helvetica,Arial,sans-serif\"] graph [ rankdir = \"TB\" ];"),
          footer: String::from("}"),
          next_node_id: 0,
          next_edge_id: 0,
          nodes: vec![],
          edges: vec![]
        }
    }

    pub fn write(&self) -> String {
        let mut content = self.header.clone();
        for i in self.nodes.iter() {
            content.push_str(i);
        }
        for i in self.edges.iter() {
            content.push_str(i);
        }

        content.push_str(&self.footer);

        content
    }

    pub fn read_tree(&mut self, tree: &Tree<i32>) {
        self.read_tree_internal(tree, "");
    }

    fn read_tree_internal(&mut self, tree: &Tree<i32>, parent_node: &str) {
        match tree {
            Tree::Empty => (),
            Tree::NonEmpty(ref node) => {
                let node_name = format!("node{}", self.next_node_id);

                let s = format!(
                    "\"{}\" [ label = <<table border='0' cellborder='1' cellspacing='0'>

                    <tr><td colspan='2'>{}</td></tr>
                    <tr><td port='f1'>left</td><td port='f2'>right</td></tr>
            
                  </table>> shape = \"none\"];",
                    node_name, node.element
                );

                self.next_node_id += 1;
                self.nodes.push(s);

                if !parent_node.is_empty() {
                    let e = format!(
                        "{} -> \"{}\":f0 [ id = {} ];",
                        parent_node, node_name, self.next_edge_id
                    );
                    self.edges.push(e);
                    self.next_edge_id += 1;
                }

                let parent_node_left = format!("\"{}\":f1", node_name);
                let parent_node_right = format!("\"{}\":f2", node_name);

                self.read_tree_internal(&node.left, &parent_node_left);
                self.read_tree_internal(&node.right, &parent_node_right);
            }
        }
    }
}
