use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::{HashMap, BTreeSet};
// use std::fs::File;

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

    pub fn contains_exact(&self, w: &[u8]) -> Option<u8> {
        let cur_node = self.search_path(w);
        if cur_node.is_some() {
            let cn_idx = cur_node.unwrap();
            let c_node = self.get_node(cn_idx);
            if c_node.nodes.is_some() {
                for n in c_node.nodes.as_ref().unwrap() {
                    let cc_node = self.get_node(*n);
                    if cc_node.symbol == 0 {
                        return Some(cc_node.meta);
                    }
                }
            }
        }
        return None;
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
    pub w_idx: u8,
    pub base_idx: u8,
    pub base_from: u8,
    pub is_flipped: bool,
}
impl UNode {
    fn new(w_idx: u8, base_idx: u8, base_from:u8, is_flipped: bool) -> UNode {
        UNode {
            w_idx: w_idx,
            base_idx: base_idx,
            base_from: base_from,
            is_flipped: is_flipped,
        }
    }
}

#[derive (Default)]
struct UsageTree {
    // Map from node to parent node
    parent_map: HashMap<UNode, UNode>,
}

impl UsageTree {
    fn new() -> UsageTree {
        UsageTree::default()
    }
    pub fn add_edge(&mut self, from: UNode, to: UNode) {
        self.parent_map.insert(to, from);
    }
    pub fn remove_node(&mut self, n: &UNode) {
        self.parent_map.remove(n);
    }
    pub fn contains_node(&self, n: &UNode) -> bool{
        return self.parent_map.contains_key(n);
    }
    pub fn get_parent(&self, n: &UNode) -> Option<&UNode> {
        self.parent_map.get(n)
    }

}

struct ResBuilder {
    pub words: Vec<u8>,
    pub len: usize,
}

impl ResBuilder {
    pub fn new() -> ResBuilder {
        ResBuilder {
            words: Vec::new(),
            len: 0,
        }
    }
    pub fn to_string(&self, input_words: &Vec<Vec<u8>>) -> String {
        let mut buf: Vec<u8> = Vec::new();
        for w in self.words
                        .iter()
                        .rev()
                        .map(|m| *m as usize) {
            let cw = input_words[w].as_slice();
            buf.extend_from_slice(cw);
        }
        String::from_utf8(buf).unwrap()
    }
}

//     pub fn set_len(&mut self, new_len: usize) {
//         self.len = new_len;
//     }

//     pub fn extend(&mut self, part: &[u8]) {
//         let start_idx = self.len;
//         self.len += part.len();
//         while self.buf.len() < self.len {
//             self.buf.push(0);
//         }
//         for i in start_idx..self.len {
//             self.buf[i] = part[i - start_idx];
//         }
//     }
//     pub fn len(&self) -> usize {
//         self.len
//     }

//     

//     pub fn split_after<'a>(&'a self, mid: usize) -> &'a [u8] {
//         let (_, first) = self.buf.split_at(mid);
//         let (res, _) = first.split_at(self.len - mid);
//         return res;
//     }
// }

struct Solver {
    input_words: Vec<Vec<u8>>,
    prefix_tree: PrefixTree,
    usage_tree: UsageTree,
    result: Option<ResBuilder>,
    verbose: bool,
}

impl Solver {
    fn build_expression(&mut self, from: UNode, possible_len: usize) {
        // self.counter += 1;
        // if self.counter > 300000 {
        //     return;
        // }
        // if self.res_builder.len() > 2000{
        //    panic!("wrong answer");
        // }

        // if self.verbose {
            // println!("start build {:?} position {}",
            //          self.res_builder.to_string(),
            //          cur_pos);
        // }
        if self.result.is_some() {
            return;
        }
        if possible_len > 20000 {
            return;
        }
        if self.verbose {
            print!("{:<1$}", "", possible_len);
            println!("build expression {:?}", possible_len);
        }

        // if self.result.is_some() {
        //     return;
        // }
        // if self.should_stop() {
        //     return;
        // }
        if self.should_stop(possible_len) {
            return;
        }
        let sub_words;
        // let cur_word;
        // let cur_usage_idx;
        // let cur_length;

        let base_word = from.base_idx as usize;
        let base_from = from.base_from as usize;

        // First detect if we can finish
        {
            let from_word = &self.input_words[base_word];
            let (_, cur_word) = from_word.split_at(base_from);

            let last_exact = self.prefix_tree.contains_exact(cur_word);

            if last_exact.is_some() {
                // build result
                
                let mut back_rev = Some(&from);
                let mut res = ResBuilder::new();
                let mut is_base_expected= true;
                if from.is_flipped {
                    res.words.push(last_exact.unwrap());
                }
                while back_rev.is_some() {
                    {
                        let child = back_rev.as_ref().unwrap();
                        if self.verbose {
                            print!("{:<1$}", "", possible_len);
                            println!("{:?}", child);
                        }
                        if child.is_flipped {
                            // res.words.push(child.w_idx);
                            is_base_expected= true;
                            // c_base = child.base_idx;
                            // if c_is_base {
                            //     let b_word = &self.input_words[c_base as usize];
                            //     let mut new_res: Vec<u8> = Vec::new();
                            //     new_res.extend_from_slice(b_word);
                            //     new_res.append(&mut res);
                            //     res = new_res;
                            // }
                        } else {
                            if is_base_expected {
                                res.words.push(child.base_idx);
                                is_base_expected = false;
                            }
                        }
                        
                        // let b_word = &self.input_words[res_word as usize];
                        
                        // if self.verbose {
                        //     let cw = String::from_utf8_lossy(res.as_slice());
                        //     println!("parent {:?} current res {:?}", child.base_idx, cw);
                        // }
                    }
                    back_rev = self.usage_tree.get_parent(back_rev.as_ref().unwrap());
                }


                // if self.result.is_some() && self.result.as_ref().unwrap().len < res.len) {
                //     return;
                // }

                // self.result = Some(self.res_builder.to_string());
                if self.verbose {
                    print!("{:<1$}", "", possible_len);
                    println!("found solution len {:?} {:?} {:?}", possible_len, res.words, res.to_string(&self.input_words));
                }
                res.len = possible_len;
                self.result = Some(res);
                
                return;
            }
            
            sub_words = self.prefix_tree.collect_exact_subwords(cur_word);
            if self.verbose {
                let cw = String::from_utf8_lossy(cur_word);
                print!("{:<1$}", "", possible_len);
                println!("sub_words for cur word {:?}: {:?}", cw, sub_words);
            }
        }
        // Next append all sub words to base word
        for (ex_word, w_len) in sub_words {
            let to = UNode::new(ex_word, from.base_idx, (base_from + w_len) as u8, from.is_flipped);
            if self.usage_tree.contains_node(&to) {
                if self.verbose {
                    print!("{:<1$}", "", possible_len);
                    println!("already exists node {:?}", to.clone());
                }
                return;
            }
            self.usage_tree.add_edge(from.clone(), to.clone());
            self.build_expression(to.clone(), possible_len);
            self.usage_tree.remove_node(&to);
        }

        // Next append all words above base
        let super_words;
        let cur_word_len;
        {
            // let cur_word = self.res_builder.split_after(cur_pos);
            // cur_word_len = cur_word.len();
            let from_word = &self.input_words[base_word];
            let (_, cur_word) = from_word.split_at(base_from);
            cur_word_len = cur_word.len();
            super_words = self.prefix_tree.get_words(cur_word);
            if self.verbose {
                let cw = String::from_utf8_lossy(cur_word);
                print!("{:<1$}", "", possible_len);
                println!("super words for cur word {:?}: {:?}", cw, super_words);

            }

        }
        if super_words.is_none() {
            return;
        }

        for cur_big_idx in super_words.unwrap().iter().map(|w| *w as usize) {
            let big_word_len = self.input_words
                                   .get(cur_big_idx)
                                   .unwrap()
                                   .len();
            let next_len = possible_len + big_word_len - cur_word_len;
            if self.should_stop(next_len) {
                continue;
            }
            let to = UNode::new(from.base_idx, cur_big_idx as u8, cur_word_len as u8, !from.is_flipped);

            if self.usage_tree.contains_node(&to) {
                return;
            }

            self.usage_tree.add_edge(from.clone(), to.clone());

            // {
            //     let (_, cur_word_suffix) = self.input_words
            //                                    .get(cur_big_idx as usize)
            //                                    .unwrap()
            //                                    .as_bytes()
            //                                    .split_at(cur_word_len);
            //     self.res_builder.extend(cur_word_suffix);
            // }

            self.build_expression(to.clone(), next_len);
            self.usage_tree.remove_node(&to);
            //self.usage_tree.remove_edge(&from, &to);
            // self.res_builder.set_len(cur_length);
        }
    }

    fn should_stop(&self, possible_len: usize) -> bool {
        if self.result.is_some() {
            if self.result.as_ref().unwrap().len <= possible_len {
                return true;
            }
            // let res = self.result.as_ref().unwrap().as_bytes();
            // if self.res_builder.len() >= res.len() {
            //     return true;
            // }
            // for i in 0..self.res_builder.len() {
            //     if self.res_builder[i] != res[i] {
            //         return true;
            //     }
            // }
        }
        return false;
    }

    pub fn find_solution(&mut self) {
        // let mut total_res: Option<String> = None;

        for sub_idx in 0..self.input_words.len() {
            let super_words;
            let cur_pos;
            {
                let cur_word = &self.input_words[sub_idx];
                cur_pos = cur_word.len();
                super_words = self.prefix_tree.get_words(cur_word);
            }

            if self.verbose {
                println!("input = {} super_words {:?}", sub_idx, super_words);
            }

            if super_words.is_none() {
                continue;
            }

            for sup_idx in super_words.unwrap().iter().map(|w| *w as usize).filter(|w| *w != sub_idx) {
                let from: UNode;
                let possible_len;
                {
                    possible_len = self.input_words.get(sup_idx).unwrap().len();
                    from = UNode::new(sub_idx as u8, sup_idx as u8, cur_pos as u8, false);
                }
                self.build_expression(from.clone(), possible_len);
                // self.res_builder.set_len(0);
            }
            // self.result.as_ref().map(|w| {
            //     if total_res.is_some() {
            //         if w.len() <= total_res.as_ref().unwrap().len() {
            //             total_res = Some(w.clone());
            //         }
            //     } else {
            //         total_res = Some(w.clone());
            //     }
            // });
            // self.result = None;
        }
        // self.result = total_res;
    }
}


fn solve(input: &mut Read, output: &mut Write, verbose: bool) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    let mut input_words: Vec<Vec<u8>> = Vec::new();

    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    // let mut buf = Vec<u8>;
    let mut word_set = BTreeSet::new();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        word_set.insert(String::from(input.trim()));
    }


    // input_words.sort();
    if verbose {
        println!("words {:?}", word_set);
    }

    let mut prefix_tree = PrefixTree::new();

    for w in word_set.into_iter().rev() {
        let idx = input_words.len() as u8;
        prefix_tree.add_word(&w, idx);
        input_words.push(w.into_bytes());
    }

    let mut solver = Solver {
        input_words: input_words,
        prefix_tree: prefix_tree,
        usage_tree: UsageTree::new(),
        result: None,
        verbose: verbose,
    };

    // for _ in 0..100 {
    solver.find_solution();
    // }

    if solver.result.is_some() {
        writeln!(output, "YES").expect("correct output");
        writeln!(output, "{}", solver.result.unwrap().to_string(&solver.input_words)).expect("correct output");
    } else {
        writeln!(output, "NO").expect("correct output");;
    }

    // println!("{}", n);


}

fn main() {
    // let mut f = File::open("../test1.txt").expect("correct test");
    // solve(&mut f, &mut io::stdout(), false);
    solve(&mut io::stdin(), &mut io::stdout(), false);
}

#[cfg(test)]
mod tests {
    use solve;
    use PrefixTree;
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
            assert_eq!(tree.contains_exact(b"abac"), Some(3));
        }

    }

    // #[test]
    // fn test_usage_tree() {
    //     let mut usage_tree = UsageGraph::new();
    //     assert!(usage_tree.add_edge(UNode::new(1, 2), UNode::new(2, 3)));
    //     assert!(!usage_tree.add_edge(UNode::new(1, 2), UNode::new(2, 3)));
    //     assert!(usage_tree.add_edge(UNode::new(0, 2), UNode::new(2, 3)));
    //     usage_tree.remove_edge(&UNode::new(1, 2), &UNode::new(2, 3));
    //     assert!(usage_tree.add_edge(UNode::new(1, 2), UNode::new(2, 3)));
    // }

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
    #[test]
    fn test_run8() {
        let test = String::from("7
x
y
abxyde
fgxyhk
ab
defg
hk
");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf, false);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "YES\nabxydefgxyhk\n");
    }
}
