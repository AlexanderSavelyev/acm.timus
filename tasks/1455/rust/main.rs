use std::io::{self, BufReader};
use std::io::prelude::*;

struct Node {
    pub symbol: u8,
    pub nodes: Option<Vec<usize>>,
}

struct PrefixTree {
    pub node_pool: Vec<Node>,
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
        let mut tree = PrefixTree { node_pool: Vec::new() };
        tree.node_pool.push(Node::new(0));
        tree
    }

    pub fn get_root(&self) -> usize {
        return 0;
    }

    pub fn add_word(&mut self, word: &str, word_idx: u8) {
        let mut cur_node = self.get_root();
        for w in word.bytes() {
            cur_node = self.insert(cur_node, w);
        }
        self.insert(cur_node, word_idx);
    }

    pub fn get_words(&self, prefix: &str) -> Vec<u8> {
        let mut res : Vec<u8> = Vec::new();
        let mut cur_node = self.get_root();

        for w in prefix.bytes() {
            let c_node = self.node_pool.get(cur_node).unwrap();
            if c_node.nodes.is_some() {
                for n in c_node.nodes.as_ref().unwrap() {
                    
                }
            }
        }

        return res;
    }

    fn insert(&mut self, parent: usize, symbol: u8) -> usize {
        let mut res: Option<usize> = None;
        {
            let mut p = self.node_pool.get_mut(parent).unwrap();
            if p.nodes.is_none() {
                p.nodes = Some(Vec::new())
            }
        }
        {
            let ref p = self.node_pool.get(parent).unwrap().nodes;
            let pn = p.as_ref().unwrap();
            for n in pn {
                if self.node_pool.get(*n).unwrap().symbol == symbol {
                    res = Some(*n);
                    break;
                }
            }
        }

        if res.is_none() {
            let n_idx = self.node_pool.len();
            self.node_pool.push(Node::new(symbol));
            let mut p = self.node_pool.get_mut(parent).unwrap().nodes.as_mut().unwrap();;
            p.push(n_idx);
            res = Some(n_idx);
        }

        return res.unwrap();
    }
}

fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    // let mut buf = Vec<u8>;
    let mut prefix_tree = PrefixTree::new();
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        println!("{:?}", input);


        prefix_tree.add_word(input.trim(), 1);
        // for c in input.trim().as_bytes() {

        // }
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
    use PrefixTree;

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

    #[test]
    fn tree_test() {
        let mut tree = PrefixTree::new();

        tree.add_word("ab", 1);
        tree.add_word("abac", 2);

        let words = tree.get_words("ab");
        assert_eq!(words, [1, 2]);

    }
}
