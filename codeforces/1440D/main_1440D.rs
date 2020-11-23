use std::collections::HashSet;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[allow(dead_code)]
struct Vertex {
    data: usize,
    nei_vert: HashSet<usize>,
    nei_edge: HashSet<usize>,
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
    vertices_set: HashSet<usize>,
    edges_set: HashSet<usize>,
}

#[allow(dead_code)]
impl Vertex {
    fn new(data: usize) -> Vertex {
        Vertex {
            data: data,
            nei_vert: HashSet::new(),
            nei_edge: HashSet::new(),
        }
    }
    fn get_num_nei(&self)->usize {
        return self.nei_vert.len();
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
    fn new() -> Graph {
        Graph {
            vertices_pool: Vec::new(),
            edges_pool: Vec::new(),
            vertices_set: HashSet::new(),
            edges_set: HashSet::new(),
        }
    }
    fn add_vertex(&mut self, data: usize) -> usize {
        let res_idx = self.vertices_pool.len();
        self.vertices_pool.push(Vertex::new(data));
        self.vertices_set.insert(res_idx);
        return res_idx;
    }
    fn add_edge(&mut self, data: usize, v1: usize, v2: usize) -> usize {
        self.vertices_pool[v1].nei_vert.insert(v2);
        self.vertices_pool[v2].nei_vert.insert(v1);
        let res_idx = self.edges_pool.len();
        self.edges_pool.push(Edge::new(data, v1, v2));
        self.edges_set.insert(res_idx);
        self.vertices_pool[v1].nei_edge.insert(res_idx);
        self.vertices_pool[v2].nei_edge.insert(res_idx);
        return res_idx;
    }
    fn get_vertex(&self, v_idx: usize) -> &Vertex{
        return &self.vertices_pool[v_idx];
    }
    fn remove_vertex(&mut self, v_idx: usize) {
        self.vertices_set.remove(&v_idx);
        let mut nei_vec: Vec<usize> = Vec::new();
        for nei in &self.vertices_pool[v_idx].nei_vert {
            nei_vec.push(*nei);
        }
        for nei in &nei_vec {
            self.vertices_pool[*nei].nei_vert.remove(&v_idx);
        }
        nei_vec.clear();
        for nei in &self.vertices_pool[v_idx].nei_edge {
            nei_vec.push(*nei);
        }
        for nei in &nei_vec {
            self.edges_set.remove(nei);
        }
    }

    fn get_num_vertices(&self) -> usize {
        return self.vertices_set.len();
    }

    fn get_vertices(&self) -> &HashSet<usize> {
        return &self.vertices_set;
    }

    fn get_component(&self, start_idx: usize) -> HashSet<usize> {
        let mut stack: HashSet<usize> = HashSet::new();
        let mut component: HashSet<usize> = HashSet::new();
        stack.insert(start_idx);

        loop {
            let next_v = stack.iter().next().cloned();
            match next_v {
                Some(next_idx) => {
                    stack.remove(&next_idx);
                    component.insert(next_idx);
                    for nei in &self.vertices_pool[next_idx].nei_vert {
                        if !component.contains(nei) {
                            stack.insert(*nei);
                        }
                    }
                },
                None => {
                    break;
                }
            }
        }
        return component;
    }
    fn get_components(&self) -> Vec<HashSet<usize>> {
        let mut res: Vec<HashSet<usize>> = Vec::new();

        let start_v: Option<usize> = self.vertices_set.iter().next().cloned();
        match start_v {
            Some(start_idx) => {
                let mut full_set: HashSet<usize> = HashSet::new();
                let mut next_idx = start_idx;
                loop {
                    let component: HashSet<usize> = self.get_component(next_idx);
                    
                    for p in &component {
                        full_set.insert(*p);
                    }
                    res.push(component);
                    if full_set.len() < self.vertices_set.len() {
                        for v in self.vertices_set.difference(&full_set) {
                            next_idx = *v;
                            break;
                        }
                    } else {
                        break;
                    }
                }
            },
            None => {
                return res;
            }
        }

        

        return res;
    }
}


fn remove_vertices_min_nei(graph: &mut Graph, min_nei: usize) {
    let mut vertices_queue: HashSet<usize> = HashSet::new();

    for i in graph.get_vertices() {
        vertices_queue.insert(*i);
    }

    loop {
        let next_v = vertices_queue.iter().next().cloned();
        match next_v {
            Some(next_idx) => {
                vertices_queue.remove(&next_idx);
                let next_vertex = graph.get_vertex(next_idx);
                if next_vertex.get_num_nei() < min_nei {
                    for nei in &next_vertex.nei_vert {
                        vertices_queue.insert(*nei);
                    }
                    graph.remove_vertex(next_idx);
                }
            },
            None => break
        }
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

        let n: usize = s.next().unwrap().trim().parse().unwrap();
        let m: usize = s.next().unwrap().trim().parse().unwrap();
        let k: usize = s.next().unwrap().trim().parse().unwrap();

        let mut graph: Graph = Graph::new();

        for i in 0..n {
            graph.add_vertex(i + 1);
        }

        for i in 0..m {
            input.clear();
            reader.read_line(&mut input).unwrap();
            let mut v = input.trim().split(' ');

            let v1: usize = v.next().unwrap().trim().parse().unwrap();
            let v2: usize = v.next().unwrap().trim().parse().unwrap();

            // println!("{} {}", v1, v2);

            graph.add_edge(i + 1, v1 - 1, v2 - 1);

        }

        remove_vertices_min_nei(&mut graph, k - 1);
        println!("1 graph.get_num_vertices() {}", graph.get_num_vertices());

        
        if graph.get_num_vertices() == 0 {
            writeln!(output, "-1").expect("correct output");
            return;
        }

        let connected_components = graph.get_components();

        let mut clique: Option<&HashSet<usize>> = None;

        for component in &connected_components {
            println!("component.len() {}", component.len());
            if component.len() == k {
                clique = Some(component);
                break;
            }
        }

        remove_vertices_min_nei(&mut graph, k);

        if graph.get_num_vertices() > 0 {
            writeln!(output, "1 {}", graph.get_num_vertices()).expect("correct output");
            let collected_vertices: Vec<String> =  graph.get_vertices().iter().map(|&v| graph.get_vertex(v).data.to_string()).collect();
            writeln!(output, "{}", collected_vertices.join(" ")).expect("correct output");
        } else {
            match clique {
                Some(vertices) => {
                    writeln!(output, "2").expect("correct output");
                    let collected_vertices: Vec<String> =  vertices.iter().map(|&v| graph.get_vertex(v).data.to_string()).collect();
                    writeln!(output, "{}", collected_vertices.join(" ")).expect("correct output");
                },
                None => {
                    writeln!(output, "-1").expect("correct output");
                }
            }
        }

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
                assert_eq!(
                    res,
                    "2
4 1 2 3 
1 10
1 2 3 4 5 6 7 8 9 10 
-1
"
        );
    }
}
