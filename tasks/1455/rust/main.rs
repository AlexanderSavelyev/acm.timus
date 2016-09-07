use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::{HashMap, BTreeSet, HashSet};

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

struct Solver {
    input_words: Vec<String>,
    prefix_tree: PrefixTree,
    usage_graph: UsageGraph,
    res_builder: Vec<u8>,
    result: Option<String>,
    verbose: bool,
}

impl Solver {
    fn build_expression(&mut self, cur_pos: usize, from: UNode) {

        // if self.res_builder.len() > 2000{
        //    panic!("wrong answer");
        // }

        if self.verbose {
            println!("start build {:?} position {}",
                     String::from_utf8_lossy(&self.res_builder),
                     cur_pos);
        }

        // if self.result.is_some() {
        //     return;
        // }
        if self.should_stop() {
            return;
        }
        let sub_words;
        let cur_usage_idx;
        let cur_length;
        {
            cur_length = self.res_builder.len();
            cur_usage_idx = cur_length - cur_pos;

            let (_, cur_word) = self.res_builder.split_at(cur_pos);

            if self.prefix_tree.contains_exact(cur_word) {
                // if self.result.is_some() && self.result.as_ref().unwrap().len() < self.res_builder.len() {
                //     return;
                // }
                self.result = Some(String::from_utf8_lossy(&self.res_builder).to_string());
                return;
            }

            sub_words = self.prefix_tree.collect_exact_subwords(cur_word);
        }
        if self.verbose {
            println!("sub_words {:?}", sub_words);
        }
        for (ex_word, w_len) in sub_words {
            let to = UNode::new(cur_usage_idx as i32, ex_word as i32);
            if !self.usage_graph.add_edge(from.clone(), to.clone()) {
                if self.verbose {
                    println!("already exists edge {:?} - {:?}", from.clone(), to.clone());
                }
                return;
            }
            self.build_expression(cur_pos + w_len, to.clone());

            self.usage_graph.remove_edge(&from, &to);
        }
        let super_words;
        let cur_word_len;
        {
            let (_, cur_word) = self.res_builder.split_at(cur_pos);
            cur_word_len = cur_word.len();
            super_words = self.prefix_tree.get_words(cur_word);
        }
        if super_words.is_none() {
            return;
        }
        for cur_big_idx in super_words.unwrap() {
            let to = UNode::new(cur_usage_idx as i32, cur_big_idx as i32);

            if !self.usage_graph.add_edge(from.clone(), to.clone()) {
                return;
            }

            {
                let (_, cur_word_suffix) = self.input_words
                                               .get(cur_big_idx as usize)
                                               .unwrap()
                                               .as_bytes()
                                               .split_at(cur_word_len);
                self.res_builder.extend_from_slice(cur_word_suffix);
            }

            self.build_expression(cur_length, to.clone());
            self.usage_graph.remove_edge(&from, &to);
            self.res_builder.truncate(cur_length);
        }
    }

    fn should_stop(&self) -> bool {
        if self.result.is_some() {
            let res = self.result.as_ref().unwrap().as_bytes();
            if self.res_builder.len() >= res.len() {
                return true;
            }
            for i in 0..self.res_builder.len() {
                if self.res_builder[i] != res[i] {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn find_solution(&mut self) {
        let mut total_res: Option<String> = None;

        for idx in 0..self.input_words.len() {
            let super_words;
            let cur_pos;
            {
                let cur_word = &self.input_words[idx].as_bytes();
                cur_pos = cur_word.len();
                super_words = self.prefix_tree.get_words(cur_word);
            }

            if self.verbose {
                println!("input = {} super_words {:?}", idx, super_words);
            }

            if super_words.is_none() {
                continue;
            }


            for sup_idx in super_words.unwrap().iter().map(|w| *w as usize).filter(|w| *w != idx) {
                {
                    let cur_super_word = self.input_words.get(sup_idx).unwrap().as_bytes();
                    self.res_builder.extend_from_slice(cur_super_word);
                }
                self.build_expression(cur_pos, UNode::new(-1, idx as i32));
                self.res_builder.clear();
            }
            self.result.as_ref().map(|w| {
                if total_res.is_some() {
                    if w.len() <= total_res.as_ref().unwrap().len() {
                        total_res = Some(w.clone());
                    }
                } else {
                    total_res = Some(w.clone());
                }
            });
            self.result = None;
        }
        self.result = total_res;
    }
}


fn solve(input: &mut Read, output: &mut Write, verbose: bool) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    let mut input_words: Vec<String> = Vec::new();

    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    // let mut buf = Vec<u8>;
    let mut word_set = BTreeSet::new();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        word_set.insert(String::from(input.trim()));
    }
    for w in word_set.into_iter().rev() {
        input_words.push(w);
    }

    // input_words.sort();
    if verbose {
        println!("words {:?}", input_words);
    }

    let mut prefix_tree = PrefixTree::new();

    for idx in 0..input_words.len() {
        prefix_tree.add_word(&input_words[idx], idx as u8);
    }
    // let mut res: Vec<u8> = Vec::new();

    let mut solver = Solver {
        input_words: input_words,
        prefix_tree: prefix_tree,
        usage_graph: UsageGraph::new(),
        res_builder: Vec::new(),
        result: None,
        verbose: verbose,
    };

    solver.find_solution();

    if solver.result.is_some() {
        writeln!(output, "YES").expect("correct output");
        writeln!(output, "{}", solver.result.unwrap()).expect("correct output");
    } else {
        writeln!(output, "NO").expect("correct output");;
    }

    // println!("{}", n);


}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout(), false);
}

#[cfg(test)]
mod tests {
    use solve;
    use PrefixTree;
    use UsageGraph;
    use UNode;
    use std::fs::File;

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
        solve(&mut test_r, &mut buf, false);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "YES\nabacbabbc\n");
    }

    #[test]
    fn tree_test1() {
        let mut tree = PrefixTree::new();

        tree.add_word("ab", 1);
        tree.add_word("abac", 2);

        let mut words = tree.get_words(b"ab").unwrap();
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
            let mut words = tree.get_words(b"a").unwrap();
            words.sort();
            assert_eq!(words, vec![0, 1, 3]);
        }
        {
            let mut words = tree.get_words(b"ab").unwrap();
            words.sort();
            assert_eq!(words, vec![0, 3]);
        }
        {
            let mut words = tree.get_words(b"bca").unwrap();
            words.sort();
            assert_eq!(words, vec![6]);
        }
        {
            let words = tree.get_words(b"bcax");
            assert_eq!(words, None);
        }
        {
            assert_eq!(tree.contains_exact(b"abac"), true);
        }

    }

    #[test]
    fn test_usage_graph() {
        let mut usage_graph = UsageGraph::new();
        assert!(usage_graph.add_edge(UNode::new(1, 2), UNode::new(2, 3)));
        assert!(!usage_graph.add_edge(UNode::new(1, 2), UNode::new(2, 3)));
        assert!(usage_graph.add_edge(UNode::new(-1, 2), UNode::new(2, 3)));
        usage_graph.remove_edge(&UNode::new(1, 2), &UNode::new(2, 3));
        assert!(usage_graph.add_edge(UNode::new(1, 2), UNode::new(2, 3)));
    }

    #[test]
    fn test_run2() {
        let test = String::from("2
ab
abab");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf, false);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "YES\nabab\n");
    }
    #[test]
    fn test_run3() {
        let test = String::from("4
ab
ba
aba
bab");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf, false);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "YES\nababa\n");
    }
    #[test]
    fn test_run4() {
        let test = String::from("3
abcab
abc
c");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf, false);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "YES\nabcabc\n");
    }
    #[test]
    fn test_run5() {
        let test = String::from("9
ab
ab
acb
acb
bc
bc
abac
babbc
babbc");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf, false);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "YES\nabacbabbc\n");
    }

    #[test]
    fn test_from_file1() {
        let mut f = File::open("../test1.txt").expect("correct test");
        let mut buf: Vec<u8> = Vec::new();
        // let testb = test.into_bytes();
        // let mut test_r = testb.as_slice();
        // let mut buf: Vec<u8> = Vec::new();
        solve(&mut f, &mut buf, false);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "YES\nabbabbc\n");
    }

    #[test]
    fn test_run6() {
        let test = String::from("4
ab
ab
ab
ab");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf, false);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "NO\n");
    }
    #[test]
    fn test_run7() {
        let test = String::from("4
abcc
ab
c
");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf, false);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "YES\nabcc\n");
    }
}
