use std::io::{self, BufReader};
use std::io::prelude::*;

use std::collections::BTreeMap;

struct Node {
    pub nodes: BTreeMap<String, Option<usize>>,
}

impl Node {
    fn new() -> Node {
        Node { nodes: BTreeMap::new() }
    }
}

struct NodeStorage {
    pub buf: Vec<Node>,
}

impl NodeStorage {
    pub fn new() -> NodeStorage {
        NodeStorage { buf: Vec::new() }
    }

    pub fn add_node(&mut self) -> usize {
        let res = self.buf.len();
        self.buf.push(Node::new());
        res
    }

    pub fn get_child_nodes(&mut self, node: Option<usize>) -> &mut BTreeMap<String, Option<usize>> {
        &mut self.buf.get_mut(node.unwrap()).unwrap().nodes
    }

    pub fn print_nodes(&self, output: &mut Write, node: &Option<usize>, level: usize) {
        if node.is_none() { 
            return
        }
        // let nodes = ;
        for (k, v) in &self.buf.get(node.unwrap()).unwrap().nodes {
            write!(output, "{:<1$}", "", level).expect("correct output");
            writeln!(output, "{}", k).expect("correct output");
            self.print_nodes(output, v, level+1);
        }
    }
}

fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut node_storage = NodeStorage::new();
    let root = node_storage.add_node();


    for _ in 0..n {
        let mut current_node: Option<usize> = Some(root);
        let mut parent: Option<usize> = None;
        let mut parent_name: Option<String> = None;
        input.clear();
        reader.read_line(&mut input).unwrap();

        for name in input.trim().split("\\") {
            // println!("{}", name);

            if current_node.is_none() && parent.is_some() {
                let next = node_storage.add_node();

                current_node = Some(next);
                let p_name = parent_name.as_ref().unwrap().clone();
                // node_storage.buf.get_mut(parent.unwrap()).unwrap().nodes.insert(p_name, Some(next));
                node_storage.get_child_nodes(parent).insert(p_name, Some(next));
            }
            if current_node.is_some() {
                // node_storage.buf.get_mut(current_node.unwrap())
                //             .unwrap()
                //             .nodes
                node_storage.get_child_nodes(current_node)
                            .entry(name.to_string())
                            .or_insert(None);
            }
            parent = current_node;
            parent_name = Some(name.to_string());
            // current_node = node_storage.buf.get_mut(current_node.unwrap()).unwrap().nodes[name];
            current_node = node_storage.get_child_nodes(current_node)[name];

        }

    }

    node_storage.print_nodes(output, &Some(root), 0usize);

    // writeln!(output, "{}", n).expect("correct output");

}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use solve;

    #[test]
    fn basic_test() {
        let mut f = File::open("../input.txt").expect("correct test");
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut f, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "GAMES\n DRIVERS\nHOME\nWIN\n SOFT\nWINNT\n DRIVERS\n SYSTEM32\n  CERTSRV\n   CERTCO~1\n    X86\n  CONFIG\n");
    }
}
