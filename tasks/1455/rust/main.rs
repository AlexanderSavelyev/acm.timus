use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

struct Node {
    pub symbol: u8,
    pub meta: u8,
    pub nodes: Option<Vec<usize>>,
}

struct PrefixTree {
    pub node_pool: Vec<Node>,
}

impl Node {
    pub fn new(s: u8, m: u8) -> Node {
        Node {
            symbol: s,
            meta: m,
            nodes: None,
        }
    }
}

impl PrefixTree {
    pub fn new() -> PrefixTree {
        let mut tree = PrefixTree { node_pool: Vec::new() };
        tree.node_pool.push(Node::new(0, 0));
        tree
    }

    pub fn get_root(&self) -> usize {
        return 0;
    }

    pub fn add_word(&mut self, word: &str, word_idx: u8) {
        let mut cur_node = self.get_root();
        for w in word.bytes() {
            cur_node = self.insert(cur_node, w, 0);
        }
        self.insert(cur_node, 0, word_idx);
    }

    fn get_leaves(&self, parent: usize) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        let mut node_stack: Vec<usize> = Vec::new();
        node_stack.push(parent);

        while !node_stack.is_empty() {
            let cur_node = node_stack.pop().unwrap();
            let c_node = self.node_pool.get(cur_node).unwrap();
            if c_node.nodes.is_some() {
                for n in c_node.nodes.as_ref().unwrap() {
                    node_stack.push(*n);
                }
            } else if c_node.symbol == 0 {
                res.push(c_node.meta);
            }
        }

        return res;
    }

    pub fn get_words(&self, prefix: &str) -> Option<Vec<u8>> {
        let mut cur_node = self.get_root();
        let mut has_path = false;

        for w in prefix.bytes() {
            has_path = false;
            let c_node = self.node_pool.get(cur_node).unwrap();
            if c_node.nodes.is_some() {
                for n in c_node.nodes.as_ref().unwrap() {
                    if self.node_pool.get(*n).unwrap().symbol == w {
                        cur_node = *n;
                        has_path = true;
                        break;
                    }
                }
            }
            if !has_path {
                break;
            }
        }

        if has_path {
            return Some(self.get_leaves(cur_node));
        }

        return None;
    }

    fn insert(&mut self, parent: usize, symbol: u8, meta: u8) -> usize {
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
            self.node_pool.push(Node::new(symbol, meta));
            let mut p = self.node_pool.get_mut(parent).unwrap().nodes.as_mut().unwrap();;
            p.push(n_idx);
            res = Some(n_idx);
        }

        return res.unwrap();
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct UNode {
    word_idx :i32,
    symb_idx: i32,
}
impl UNode {
    fn new(w:i32, s:i32)->UNode {
        UNode {
            word_idx:w,
            symb_idx:s,
        }
    }
}

#[derive (Default)]
struct UsageGraph {
    adj_matrix: HashMap<UNode, HashSet<UNode> >
}

impl UsageGraph {
    fn new() ->UsageGraph {
        UsageGraph::default()
    }
    fn add_edge(&mut self, from: UNode, to: UNode) ->bool {
        let mut from_set = self.adj_matrix.entry(from).or_insert(HashSet::new());
        if from_set.contains(&to) {
            return false;
        }
        from_set.insert(to);
        return true;
    }
    fn remove_edge(&mut self, from: UNode, to: UNode) {
        let mut from_set = self.adj_matrix.entry(from).or_insert(HashSet::new());
        from_set.remove(&to);
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
    use UsageGraph;
    use UNode;

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
    fn tree_test1() {
        let mut tree = PrefixTree::new();

        tree.add_word("ab", 1);
        tree.add_word("abac", 2);

        let mut words = tree.get_words("ab").unwrap();
        words.sort();
        assert_eq!(words, vec![1, 2]);

    }
    #[test]
    fn tree_test2() {
        let mut tree = PrefixTree::new();

        let test = vec!["ab", "acb", "bc", "abac", "babbc", "xwz", "bcab"];

        for i in 0..test.len() {
            tree.add_word(test[i], i as u8);
        }
        {
            let mut words = tree.get_words("a").unwrap();
            words.sort();
            assert_eq!(words, vec![0, 1, 3]);
        }
        {
            let mut words = tree.get_words("ab").unwrap();
            words.sort();
            assert_eq!(words, vec![0, 3]);
        }
        {
            let mut words = tree.get_words("bca").unwrap();
            words.sort();
            assert_eq!(words, vec![6]);
        }
        {
            let words = tree.get_words("bcax");
            assert_eq!(words, None);
        }

    }

    #[test]
    fn test_usage_graph() {
        let mut usage_graph = UsageGraph::new();
        assert!(usage_graph.add_edge(UNode::new(1,2), UNode::new(2,3)));
        assert!(!usage_graph.add_edge(UNode::new(1,2), UNode::new(2,3)));
        assert!(usage_graph.add_edge(UNode::new(-1,2), UNode::new(2,3)));
        usage_graph.remove_edge(UNode::new(1,2), UNode::new(2,3));
        assert!(usage_graph.add_edge(UNode::new(1,2), UNode::new(2,3)));
    }
}
