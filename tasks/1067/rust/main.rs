use std::io::{self, BufReader};
use std::io::prelude::*;

use std::collections::BTreeMap;

struct Node {
    pub nodes: BTreeMap<String, Option<usize>>
}

impl Node {
    fn new()->Node {
        Node {
            nodes: BTreeMap::new()
        }
    }
}

fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut node_storage: Vec<Node> = Vec::new();
    let root = node_storage.len();
    node_storage.push(Node::new());

    for _ in 1..n {
        let mut currentNode: Option<usize> = Some(root);
        let mut parent: Option<usize> = None;
        let mut parentName:Option<String> = None;
        input.clear();
        reader.read_line(&mut input).unwrap();

        for name in input.trim().split("\\") {
            println!("{}", name);

            if currentNode.is_none() && parent.is_some() {
                let next = node_storage.len();
                node_storage.push(Node::new());
                currentNode = Some(next);
                let p_name = parentName.as_ref().unwrap().clone();
                node_storage.get_mut(parent.unwrap()).unwrap().nodes.insert(p_name, Some(next));
            }
            if currentNode.is_some() {
                node_storage.get_mut(currentNode.unwrap()).unwrap().nodes.entry(name.to_string()).or_insert(None);
            }
            parent = currentNode;
            currentNode = node_storage.get_mut(currentNode.unwrap()).unwrap().nodes[name];

        }

        // let a_str = s.next().unwrap();
        // let a: i32 = a_str.trim().parse().unwrap();

        // let b_str = s.next().unwrap();
        // let b: i32 = b_str.trim().parse().unwrap();

        // println!("{} {}", a,b);
    }

    println!("{}", n);


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
        //assert_eq!(res,
//                   "2297.0716
//936297014.1164
//0.0000
//37.7757
//");
    }
}
