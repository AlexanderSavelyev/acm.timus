use std::io::{self, BufReader};
use std::io::prelude::*;

struct Node {
    pub symbol: u8,
    pub nodes: Option<Vec<usize>>,
}

struct PrefixTree {
    pub nodes: Vec<Node>,
}

impl Node {
    pub fn new(s: u8) -> Node {
        Node {
            symbol: s,
            nodes: None,
        }
    }
}

impl PrefixTree {
    pub fn new() -> PrefixTree {
        let mut tree = PrefixTree { nodes: Vec::new() };
        tree.nodes.push(Node::new(0));
        tree
    }
    pub fn get_root(&self) -> usize {
        0usize
    }

    pub fn insert(&mut self, parent: usize, symbol: u8) -> usize {
        let mut append = false;
        let mut res = 0;
        {
            let p = &self.nodes[parent];
            if p.nodes.is_none() {
                append = true;
            }
        }
        if append {
            res = self.nodes.len();
            self.nodes.push(Node::new(symbol));
            let mut p = self.nodes.get_mut(parent).unwrap();
            p.nodes = Some(Vec::new())
        }
        let ref p = self.nodes.get(parent).unwrap().nodes;
        let  pn = p.as_ref().unwrap();
        for n in pn {
            if self.nodes.get(*n).unwrap().symbol == symbol {
                res = *n;
            }
        }

        if res == 0 {

        }

        return res;

        //     let mut n = &self.nodes[parent];
        //     let idx = self.nodes.len() as u16;
        //     self.nodes.push(node);
        //     match n.nodes {
        //         Some(mut v)=>v.push(idx),
        //         None=>{n.nodes = Some(Vec::new())}
        //     }
        //     //n.nodes.push(idx);
        //     idx
    }
}

fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    // let mut buf = Vec<u8>;
    let mut root = Node::new(0);
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        println!("{:?}", input);

        for c in input.trim().as_bytes() {

        }
        // let mut s = input.trim().split(' ');

        // let a_str = s.next().unwrap();
        // let a: i32 = a_str.trim().parse().unwrap();

        // let b_str = s.next().unwrap();
        // let b: i32 = b_str.trim().parse().unwrap();

        // println!("{} {}", a,b);
    }

    // println!("{}", n);


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
        let test = String::from("6
ab
acb
bc
abac
babbc
xwz");
        // let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        // assert_eq!(res,
        //                   "2297.0716
        // 936297014.1164
        // 0.0000
        // 37.7757
        // ");
    }
}
