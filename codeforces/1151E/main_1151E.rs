use std::io::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, BufReader};

#[derive(Clone)]
struct Vertex {
    left: Option<usize>,
    right: Option<usize>,
}

impl Vertex {
    fn new() -> Vertex {
        Vertex {
            left: None,
            right: None,
        }
    }

    fn remove_nei(&mut self, nei: usize) {
        if self.left.filter(|v| *v == nei).is_some() {
            self.left.take();
        }
        if self.right.filter(|v| *v == nei).is_some() {
            self.right.take();
        }
    }
}

#[derive(Clone)]
struct Tree {
    vertices: Vec<Option<Vertex>>,
    num_components: usize,
    num_vertices: usize,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            vertices: Vec::new(),
            num_components: 1,
            num_vertices : 0,
        }
    }

    fn add_vertex(&mut self, a: usize) -> usize {
        let v_idx = self.vertices.len();
        self.vertices.push(Some(Vertex::new()));
        self.num_vertices += 1;
        
        return v_idx;
    }

    fn add_edge(&mut self, left: usize, right: usize) {
        match self.vertices[left].as_mut() {
            Some(v) => {
                v.right.replace(right);
            },
            None => {
            }
        }

        match self.vertices[right].as_mut() {
            Some(v) => {
                v.left.replace(left);
            },
            None => {
            }
        }

    }

    fn remove_vertex_by_a(&mut self, a: usize, vert_map: &BTreeMap<usize, Vec<usize>>) {
        let mut vertices_to_update: Vec<usize> = Vec::new(); 
        let vertices = &mut self.vertices;
        match vert_map.get(&a) {
            Some(a_vert) => {
                for v_idx in a_vert {
                    vertices_to_update.clear();
                    match &vertices[*v_idx] {
                        Some(vertex) => {
                            vertex.left.map(|nei| vertices_to_update.push(nei));
                            vertex.right.map(|nei| vertices_to_update.push(nei));
                        }, None => {
                        }
                    }
                    for v in &vertices_to_update {
                        vertices[*v].as_mut().map(|v1| v1.remove_nei(*v_idx));
                    }
                    if vertices_to_update.len() == 0 {
                        self.num_components -= 1;
                    } else if vertices_to_update.len() == 2 {
                        self.num_components += 1;
                    }
                    vertices[*v_idx].take();
                }
                
            }, None => {
            }
        }
        self.num_vertices -= 1;
        // self.vert_map.remove(&a);
    }

    fn copy(&mut self, other: &Tree) {
        self.num_components = other.num_components;
        self.num_vertices = other.num_vertices;

        for i in 0..other.vertices.len() {
            self.vertices[i] = other.vertices[i].clone();
        }
    }

}
fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut s = input.trim().split(' ');

    let mut init_tree: Tree = Tree::new();
    let mut prev_vertex: Option<(usize, usize)> = None;
    let mut vert_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    for _ in 0..n {
        let a: usize = s.next().unwrap().trim().parse().unwrap();
        
        match prev_vertex {
            Some((prev_v, prev_a)) => {
                if prev_a != a {
                    let v_idx = init_tree.add_vertex(a);
                    vert_map.entry(a).or_insert(Vec::new()).push(v_idx);
                    init_tree.add_edge(prev_v, v_idx);
                    prev_vertex.replace((v_idx, a));
                }
            },
            None =>{
                let v_idx = init_tree.add_vertex(a);
                vert_map.entry(a).or_insert(Vec::new()).push(v_idx);
                prev_vertex.replace((v_idx, a));
            }
        }
        
        // println!("{}", a);
    }

    let mut num_components = 0_usize;
    let mut num_last_components;
    // let mut num_diag_components = 0_usize;

    // for (_, v) in &init_tree.vert_map {
    //     num_diag_components += v.len();
    // }
    // println!("num_diag_components {}", num_diag_components);
    let mut prev_last_idx =  n + 1;
    let mut back_iter = vert_map.keys();
    let mut tree = init_tree.clone();
    loop {
        let last = back_iter.next_back();
        match last {
            Some(last_v) => {
                let mut v_iter = vert_map.keys();
                tree.copy(&init_tree);
                let mut prev_first_idx = 0_usize;
                num_last_components = 0_usize;
                loop {
                    let first = v_iter.next();
                    match first {
                        Some(first_v) => {
                            // println!("check {} {}", *first_v, *last_v);
                            // println!("tree.num_components {}", tree.num_components);
                            num_components += tree.num_components * (*first_v - prev_first_idx);
                            num_last_components += tree.num_components * (*first_v - prev_first_idx);
                            if *first_v == *last_v {
                                // println!("break");
                                break;
                            }
                            tree.remove_vertex_by_a(*first_v, &vert_map);
                            prev_first_idx = *first_v;
                        },
                        None => {
                            break;
                        }
                    }
                }
                // println!("calculate last {}", num_last_components * (prev_last_idx - last_v - 1));
                num_components += num_last_components * (prev_last_idx - last_v - 1);
                init_tree.remove_vertex_by_a(*last_v, &vert_map);
                prev_last_idx = *last_v;
                if init_tree.num_vertices == 0 {
                    break;
                }
                // if init_tree.vert_map.len() == 0 {
                //     break;
                // }
            }, None => {
                break;
            }
        }
        
        
    }

    writeln!(output, "{}", num_components).expect("correct output");
    // println!("{}", n);
}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use solve;

    #[test]
    fn basic_test1() {
        let test = String::from(
            "3
2 1 3",
        );
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "7
");
    }
    #[test]
    fn basic_test2() {
        let test = String::from(
            "4
2 1 1 3",
        );
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "11
");
    }
    #[test]
    fn basic_test3() {
        let test = String::from(
            "10
1 5 2 5 5 3 10 6 5 1",
        );
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "104
");
    }
}
