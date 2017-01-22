use std::io::{self, BufReader};
use std::io::prelude::*;

#[derive (Default, Debug)]
struct Meta {
    pub level: usize,
    pub idx: usize,
}
#[derive (Debug)]
struct Node {
    pub meta: Meta,
    pub nodes: Option<Vec<usize>>,
}

struct Tree {
    pub node_pool: Vec<Node>,
}

impl Node {
    pub fn new(m: Meta) -> Node {
        Node {
            meta: m,
            nodes: None,
        }
    }
}

impl Tree {
    pub fn new() -> Tree {
        let mut tree = Tree { node_pool: Vec::new() };
        tree.node_pool.push(Node::new(Meta::default()));
        tree
    }

    fn get_root(&self) -> usize {
        return 0;
    }

    fn get_node<'a>(&'a self, n_idx: usize) -> &'a Node {
        self.node_pool.get(n_idx).unwrap()
    }

    fn insert_edge(&mut self, parent: usize, child: usize)  {
        {
            let mut p = self.node_pool.get_mut(parent).unwrap();
            if p.nodes.is_none() {
                p.nodes = Some(Vec::new())
            }
        }
        let mut p = self.node_pool.get_mut(parent).unwrap().nodes.as_mut().unwrap();
        p.push(child);
    }

    fn insert_node(&mut self, meta: Meta) -> usize {
        let res = self.node_pool.len();
        self.node_pool.push(Node::new(meta));
        return res
    }

    // fn has_path(&self, start: usize, end: usize) -> bool {
    //     let mut has_path = false;
    //     let mut node_stack: Vec<usize> = Vec::new();
    //     node_stack.push(start);

    //     while !node_stack.is_empty() && !has_path {
    //         let cur_node = node_stack.pop().unwrap();
    //         let c_node = self.get_node(cur_node);
    //         if c_node.nodes.is_some() {
    //             for n in c_node.nodes.as_ref().unwrap() {
    //                 node_stack.push(*n);
    //                 if *n == end {
    //                     has_path = true;
    //                 }
    //             }
    //         }
    //     }

    //     return has_path;
    // }
    fn traverse_path(&self, cur_node: usize, end: usize, path: &mut Vec<usize>) -> bool {
        let c_node = self.get_node(cur_node);
        if c_node.nodes.is_some() {
            for n in c_node.nodes.as_ref().unwrap() {
                path.push(*n);
                if *n == end {
                    return true;
                }
                let res = self.traverse_path(*n, end, path);
                if res  {
                    return true;
                }
                path.pop();
            }
        } 
        return false;
    }

    fn search_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        let mut res: Vec<usize> = Vec::new();
        //println!("Start search from {:?} to {:?}", start, end);
        //res.push(start);
        let has_path = self.traverse_path(start, end, &mut res);
        if has_path {
            return Some(res);
        }
        return None;
    }
    fn search_path2(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        let mut res: Vec<usize> = Vec::new();
        res.push(start);
        let mut cur_node = start;
        let mut has_path = false;

        while !has_path {
            let c_node = self.get_node(cur_node);
            if c_node.nodes.is_some() {
                for n in c_node.nodes.as_ref().unwrap() {
                    res.push(*n);
                    if *n == end {
                        has_path = true;
                    }
                    cur_node = *n;
                    break;
                }
            }
        }
        if has_path {
            return Some(res);
        }
        return None;
    }

}

fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut lines: Vec<String> = Vec::new();
    let mut words: Vec<Vec<String>> = Vec::new();
    let mut word_nodes: Vec<Vec<usize>> = Vec::new();
    if n == 0 {
        write!(output, "IMPOSSIBLE").expect("correct output");
    }
    for _ in 0 .. n {
        let mut next_line = String::new();
        reader.read_line(&mut next_line).unwrap();
        lines.push(next_line);
    }
    input.clear();
    reader.read_line(&mut input).unwrap();
    let order = input.trim().split(' ');
    for o in order {
        let mut next: Vec<String> = Vec::new();
        let b: usize = o.trim().parse().unwrap();
        let next_input = &lines[b - 1];
        let n_spl = next_input.trim().split(' ');
        for ns in n_spl {
            next.push(ns.to_string());
        }
        words.push(next);
        word_nodes.push(Vec::new());
    }
    //println!("{:?}", words);

    //Start building tree
    let mut tree = Tree::new();

    let root = tree.get_root();

    if words.len() == 1 {
        writeln!(output, "{}", words[0][0]).expect("correct output");
        return;
    }

    for level in 0 .. words.len() - 1 {
        let n_level = level + 1;

        let w = &words[level];
        let n_w = &words[n_level];
        word_nodes[level].resize(w.len(), 0);
        word_nodes[n_level].resize(n_w.len(), 0);
        let mut no_parent = true;
        for idx in 0 .. w.len() {
            if level == 0 {
                word_nodes[level][idx] = tree.insert_node(Meta{level:level, idx: idx});
                tree.insert_edge(word_nodes[level][idx], root);
            }
            if word_nodes[level][idx] == 0 {
                continue;
            }
            for n_idx in 0 .. n_w.len() {
                if w[idx].as_str() < n_w[n_idx].as_str() {
                    no_parent = false;
                    if word_nodes[n_level][n_idx] == 0 {
                        word_nodes[n_level][n_idx] = tree.insert_node(Meta{level:n_level, idx: n_idx});
                    }
                    tree.insert_edge(word_nodes[n_level][n_idx], word_nodes[level][idx]);
                }
            }
        }

        if no_parent {
            write!(output, "IMPOSSIBLE").expect("correct output");
            return;
        }
    }

    // We have a path
    let last_level = word_nodes.len() - 1;
    for w in &word_nodes[last_level] {
        if *w > 0 {
            let path = tree.search_path2(*w, 0);
            //println!("{:?}", path);
            if path.is_some() {
                let mut p = path.unwrap();
                p.pop();
                for w in p.iter().rev() {
                    let n = tree.get_node(*w);
                    writeln!(output, "{}", words[n.meta.level][n.meta.idx]).expect("correct output");
                }
                return;
            }
        }
    }
    write!(output, "IMPOSSIBLE").expect("correct output");
}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use solve;

    #[test]
    fn test_str() {
        let a: &str ="aa";
        let b: &str ="aaaa";
        let c: &str ="Aa";
        assert_eq!(true, a < b);
        assert_eq!(false, a > b);
        assert_eq!(false, a == b);
        assert_eq!(false, a < c);
    }

    #[test]
    fn basic_test() {
        let test = String::from("11
cipher grille kamkohob
names codenames codes
newtests rejudge timus
size volume summit
watchmen braineater twosides
solution random yesorno
keywords subversion commands
bosses shooting shaitan
game strategy playgame
mnemonic palindromes bestname
eligibility rectangle rules
2 1 7 10 9 6 11 3 8 4 5");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "codenames
grille
keywords
mnemonic
playgame
random
rectangle
rejudge
shooting
size
watchmen
");
    }

    #[test]
    fn basic_test2() {
        let test = String::from("3
problems in the
first sample are
ordered not randomly
1 2 3");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "IMPOSSIBLE");
    }
#[test]
    fn basic_test3() {
        let test = String::from("3
bbb bb b
bbb bb b
bbb bb b
1 2 3");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "b
bb
bbb
");
    }
#[test]
    fn basic_test4() {
        let test = String::from("2
cipher grille kamkohob
ciphez grillz kamkohoz
2 1");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "ciphez
grille
");
    }
#[test]
    fn basic_test5() {
        let test = String::from("17
cipher grille kamkohob
names grillee pcodes
newtests rejudge timus
size volume summit
watchmen braineater twosides
solution random yesorno
keywords subversion commands
bosses shooting shaitan
game strategy playgame
mnemonic palindromes bestname
eligibility rectangle rules
txxxxxxx txxxyzasd txxxxxas
txxxxxbx txxxyzabd txxxxxbs
volvo vlot volt
vvv vvilia vvobla
what is it
zina whashing potatoes
2 1 7 10 9 6 11 3 8 4 5 12 13 14 15 16 17");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "grillee
kamkohob
keywords
mnemonic
playgame
random
rectangle
rejudge
shooting
size
twosides
txxxxxas
txxxxxbx
volvo
vvv
what
zina
");
    }
#[test]
    fn basic_test6() {
        let test = String::from("1
bbb bb b
1");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "bbb
");
    }
}
