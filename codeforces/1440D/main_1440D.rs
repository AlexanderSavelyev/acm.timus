use std::collections::HashSet;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[allow(dead_code)]
struct Vertex {
    data: usize,
    nei_edge: HashSet<usize>,
    nei_vert: HashSet<usize>,
}

#[allow(dead_code)]
struct Edge {
    data: usize,
    v1: usize,
    v2: usize,
}

#[allow(dead_code)]
struct Graph {
    vertices_pool: Vec<Vertex>,
    edges_pool: Vec<Edge>,
}

#[allow(dead_code)]
impl Vertex {
    fn new(data: usize) -> Vertex {
        Vertex {
            data: data,
            nei_edge: HashSet::new(),
            nei_vert: HashSet::new(),
        }
    }
}

#[allow(dead_code)]
impl Edge {
    fn new(data: usize, v1: usize, v2: usize) -> Edge {
        Edge {
            data: data,
            v1: v1,
            v2: v2,
        }
    }
}

#[allow(dead_code)]
impl Graph {
    fn add_vertex(&mut self, data: usize) -> usize {
        let res_idx = self.vertices_pool.len();
        self.vertices_pool.push(Vertex::new(data));
        return res_idx;
    }
    fn add_edge(&mut self, data: usize, v1: usize, v2: usize) -> usize {
        self.vertices_pool[v1].nei_vert.insert(v2);
        self.vertices_pool[v2].nei_vert.insert(v1);
        let res_idx = self.edges_pool.len();
        self.edges_pool.push(Edge::new(data, v1, v2));
        self.vertices_pool[v1].nei_edge.insert(res_idx);
        self.vertices_pool[v2].nei_edge.insert(res_idx);
        return res_idx;
    }
}

fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();
        // println!("{:?}", input);
        let mut s = input.trim().split(' ');

        let n_str = s.next().unwrap();
        let n: usize = n_str.trim().parse().unwrap();

        let k_str = s.next().unwrap();
        let k: usize = k_str.trim().parse().unwrap();

        input.clear();
        reader.read_line(&mut input).unwrap();

        // let elements = input.trim().split(' ');
        // println!("{} {}", n, k);

        // println!("sum {}", max_sum);
        // writeln!(output, "{}", max_sum).expect("correct output");
    }
}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use solve;

    #[test]
    fn basic_test() {
        let test = String::from(
            "3
            5 9 4
            1 2
            1 3
            1 4
            1 5
            2 3
            2 4
            2 5
            3 4
            3 5
            10 15 3
            1 2
            2 3
            3 4
            4 5
            5 1
            1 7
            2 8
            3 9
            4 10
            5 6
            7 10
            10 8
            8 6
            6 9
            9 7
            4 5 4
            1 2
            2 3
            3 4
            4 1
            1 3",
        );
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
//         assert_eq!(
//             res,
//             "165
// 108
// 145
// 234
// 11
// 3
// "
        );
    }
}
