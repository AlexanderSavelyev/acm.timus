use std::collections::{HashSet, HashMap};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::hash_set::Iter;
use std::cmp;

#[allow(dead_code)]
#[derive(Clone)]
struct Vertex {
    data: usize,
    nei_vert: HashSet<usize>,
    nei_edge: HashSet<usize>,
}

#[allow(dead_code)]
#[derive(Clone)]
struct Edge {
    data: usize,
    v1: usize,
    v2: usize,
}

#[allow(dead_code)]
#[derive(Clone)]
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
        // println!("remove vertex {}", v_idx);
        // println!("size before {}", self.vertices_set.len());
        self.vertices_set.remove(&v_idx);
        // println!("size before {}", self.vertices_set.len());

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
    fn get_edges(&self) -> &HashSet<usize> {
        return &self.edges_set;
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
