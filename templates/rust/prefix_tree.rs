use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::{HashMap, BTreeSet, HashSet};
use std::fs::File;

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

    fn get_root(&self) -> usize {
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
            let c_node = self.get_node(cur_node);
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

    // returns node if exists

    fn search_path(&self, prefix: &[u8]) -> Option<usize> {
        let mut cur_node = self.get_root();
        let mut has_path = false;

        for w in prefix {
            has_path = false;
            let c_node = self.get_node(cur_node);
            if c_node.nodes.is_some() {
                for n in c_node.nodes.as_ref().unwrap() {
                    if self.get_node(*n).symbol == *w {
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
            return Some(cur_node);
        }

        return None;
    }

    pub fn get_words(&self, prefix: &[u8]) -> Option<Vec<u8>> {
        let cur_node = self.search_path(prefix);
        return cur_node.map(|n| self.get_leaves(n));
    }

    fn get_node<'a>(&'a self, n_idx: usize) -> &'a Node {
        self.node_pool.get(n_idx).unwrap()
    }

    pub fn contains_exact(&self, w: &[u8]) -> bool {
        let cur_node = self.search_path(w);
        if cur_node.is_some() {
            let cn_idx = cur_node.unwrap();
            let c_node = self.get_node(cn_idx);
            if c_node.nodes.is_some() {
                for n in c_node.nodes.as_ref().unwrap() {
                    let cc_node = self.get_node(*n);
                    if cc_node.symbol == 0 {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn collect_exact_subwords(&self, prefix: &[u8]) -> Vec<(u8, usize)> {
        let mut res = Vec::new();
        let mut cur_node = self.get_root();
        let mut has_path;
        let mut w_len: usize = 0;

        for w in prefix {
            has_path = false;
            let c_node = self.get_node(cur_node);
            if c_node.nodes.is_some() {
                for n in c_node.nodes.as_ref().unwrap() {
                    let cc_node = self.get_node(*n);
                    if cc_node.symbol == *w {
                        cur_node = *n;
                        has_path = true;
                    }
                    if cc_node.symbol == 0 {
                        res.push((cc_node.meta, w_len));
                    }
                }
            }
            if !has_path {
                break;
            }
            w_len += 1;
        }

        return res;
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
                if self.get_node(*n).symbol == symbol {
                    res = Some(*n);
                    break;
                }
            }
        }

        if res.is_none() {
            let n_idx = self.node_pool.len();
            self.node_pool.push(Node::new(symbol, meta));
            let mut p = self.node_pool.get_mut(parent).unwrap().nodes.as_mut().unwrap();
            p.push(n_idx);
            res = Some(n_idx);
        }

        return res.unwrap();
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct UNode {
    word_idx: i32,
    symb_idx: i32,
}
impl UNode {
    fn new(w: i32, s: i32) -> UNode {
        UNode {
            word_idx: w,
            symb_idx: s,
        }
    }
}

#[derive (Default)]
struct UsageGraph {
    adj_matrix: HashMap<UNode, HashSet<UNode>>,
}

impl UsageGraph {
    fn new() -> UsageGraph {
        UsageGraph::default()
    }
    fn add_edge(&mut self, from: UNode, to: UNode) -> bool {
        let mut from_set = self.adj_matrix.entry(from).or_insert(HashSet::new());
        if from_set.contains(&to) {
            return false;
        }
        from_set.insert(to.clone());
        return true;
    }
    fn remove_edge(&mut self, from: &UNode, to: &UNode) {
        self.adj_matrix.get_mut(from).map(|from_set| from_set.remove(to));
    }
}

struct ResBuilder {
    buf: Vec<u8>,
    len: usize,
}

impl ResBuilder {
    pub fn new() ->ResBuilder {
        ResBuilder {
            buf: Vec::new(),
            len: 0,
        }
    }

    pub fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }

    pub fn extend(&mut self, part: &[u8]) {
        let start_idx = self.len;
        self.len += part.len();
        while self.buf.len() < self.len {
            self.buf.push(0);
        }
        for i in start_idx..self.len {
            self.buf[i] = part[i-start_idx];
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn to_string(&self)->String {
        let (res, _) = self.buf.split_at(self.len);
        String::from_utf8_lossy(res).to_string()
    }

    pub fn split_after<'a>(&'a self, mid: usize) -> &'a [u8] {
        let (_, first) = self.buf.split_at(mid);
        let (res, _) = first.split_at(self.len-mid);
        return res 
    }
}