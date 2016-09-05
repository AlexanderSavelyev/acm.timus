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
            w_len += 1;
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
    prefix_tree: PrefixTree,
    usage_graph: UsageGraph,
    res_builder: Vec<u8>,
    result: Option<String>,
}

impl Solver {
    fn build_expression(&mut self, cur_pos: usize, from: UNode) {
        if self.result.is_some() {
            return;
        }
        //      if(result != null && (t.length() >= result.length() || result.length() > 1000)) {
        //         return;
        //      }
        let sub_words;
        let cur_usage_idx;
        {
            let cur_length = self.res_builder.len();
            cur_usage_idx = cur_length - cur_pos;

            let (_, cur_word) = self.res_builder.split_at(cur_pos);

            if self.prefix_tree.contains_exact(cur_word) {
                self.result = Some(String::from_utf8_lossy(&self.res_builder).to_string());
                return;
            }

            sub_words = self.prefix_tree.collect_exact_subwords(cur_word);
        }
        for (ex_word, w_len) in sub_words {
            let to = UNode::new(cur_usage_idx as i32, ex_word as i32);
            if !self.usage_graph.add_edge(from.clone(), to.clone()) {
                return;
            }
            self.build_expression(cur_pos + w_len, to.clone());
        }



        //       LinkedList<Integer> w1 = getAllWordsContainedPrefix(curWord);
        //       for (Integer curBigIdx : w1) {
        //          String curBigWord = words.get(curBigIdx).substring(curWord.length());

        // //         System.out.println("len = " + curUsageIdx + " w = " + curBigIdx);
        //          Pair<Integer, Integer> to = new Pair(curUsageIdx, curBigIdx);
        //          if (!usageTree.addEdge(curUsage, to)) {
        //             return;
        //          }
        //          t.append(curBigWord);
        //          buildExperssion(t, curLength, to);
        //          usageTree.removeEdge(curUsage, to);
        //          t.setLength(curLength);
        //       }
    }
}


fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    let mut input_words: Vec<String> = Vec::new();

    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    // let mut buf = Vec<u8>;

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        println!("{:?}", input);
        input_words.push(String::from(input.trim()));
    }

    input_words.sort();

    let mut exact_words: HashMap<String, usize> = HashMap::new();
    let mut prefix_tree = PrefixTree::new();

    for idx in 0..input_words.len() {
        prefix_tree.add_word(&input_words[idx], idx as u8);
        exact_words.insert(input_words.get(idx).unwrap().clone(), idx);
    }
    // let mut res: Vec<u8> = Vec::new();

    let mut solver = Solver {
        prefix_tree: prefix_tree,
        usage_graph: UsageGraph::new(),
        res_builder: Vec::new(),
        result: None,
    };

    for idx in 0..input_words.len() {

        let sup_words = solver.prefix_tree.get_words(&input_words[idx].as_bytes());

        if sup_words.is_none() {
            continue;
        }

        for sup_idx in sup_words.unwrap() {
            let cur_super_word = input_words.get(sup_idx as usize).unwrap().as_bytes();
            // res.extend_from_slice(cur_super_word);
            solver.res_builder.extend_from_slice(cur_super_word);
            let cur_pos = cur_super_word.len();

            solver.build_expression(cur_pos, UNode::new(-1, idx as i32));

            solver.res_builder.clear();

        }

    }



    if solver.result.is_some() {
        writeln!(output, "YES").expect("correct output");
        writeln!(output, "{}", solver.result.unwrap()).expect("correct output");
    } else {
        writeln!(output, "NO").expect("correct output");;
    }

    // println!("{}", n);


}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
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
        assert_eq!(res, "NO\n");
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
}
