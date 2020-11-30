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


// fn build_inverted_graph(graph: &Graph) -> Graph{
//     let mut res: Graph = Graph::new();
//     for i in 0 .. graph.vertices_pool.len() {
//         res.add_vertex(i + 1);
//     }
//     for v1 in graph.get_vertices() {
//         for v2 in graph.get_vertices() {
//             if !graph.contains_edge(*v1, *v2) {
//                 res.add_edge(0, *v1, *v2);
//             }
//         }
//     }
//     return res;
// }
#[allow(dead_code)]
fn traverse_search(graph: &Graph, clique: &mut HashSet<usize>, vert: usize, k: usize, visited: &HashSet<usize>) {
    for nei in &graph.get_vertex(vert).nei_vert {
        if visited.contains(&nei) {
            continue;
        }
        if clique.contains(&nei) {
            continue;
        }
        if clique.is_subset(&graph.get_vertex(*nei).nei_vert) {
            clique.insert(*nei);
            if clique.len() == k {
                return;
            }
            traverse_search(graph, clique, *nei, k, visited);
            if clique.len() == k {
                return;
            } else {
                clique.remove(&nei);
            }
        }
    }
}

#[allow(dead_code)]
fn find_clique_max(graph: &Graph, k: usize) -> Option<HashSet<usize>> {
    let mut res: Option<HashSet<usize>> = None;
    let mut clique: HashSet<usize> = HashSet::new();
    for v1 in graph.get_vertices() {
        if graph.get_vertex(*v1).nei_vert.len() != k-1 {
            continue;
        }
        // if component.len() - visited_vert.len() < k {
        //     continue;
        // }
        let mut visited_vert: HashSet<usize> = HashSet::new();
        let clique_candidates: &HashSet<usize> = &graph.get_vertex(*v1).nei_vert;
        clique.insert(*v1);
        // traverse_search(graph, &mut clique, *v1, k, &visited_vert);

        let mut current_level: usize = 0;
        let mut iter_stack: Vec<Iter<usize>> = Vec::new();
        let mut clique_stack: Vec<usize> = Vec::new();

        iter_stack.push(graph.get_vertex(*v1).nei_vert.iter());
        loop {
            let current_idx = iter_stack[current_level].next();
            match current_idx {
                Some(nei) => {
                    if !clique_candidates.contains(nei) {
                        continue;
                    }
                    if clique.contains(nei) {
                        continue;
                    }
                    if visited_vert.contains(nei) {
                        continue;
                    }
                    if clique.is_subset(&graph.get_vertex(*nei).nei_vert) {
                        clique.insert(*nei);
                        clique_stack.push(*nei);
                        if clique.len() == k {
                            res = Some(clique);
                            return res;
                        } else {
                            current_level += 1;
                            iter_stack.push(graph.get_vertex(*nei).nei_vert.iter())
                        }
                    }
                },
                None => {
                    iter_stack.pop();
                    current_level -= 1;
                    let last_item = clique_stack.pop();
                    match last_item {
                        Some(v) => {
                            clique.remove(&v);
                        },
                        None => {
                            break;
                        }
                    }
                }
            }
        }
        clique.remove(v1);
        visited_vert.insert(*v1);
    }

    return res;
}

#[allow(dead_code)]
fn find_clique(graph: &mut Graph, k: usize) -> Option<HashSet<usize>> {
    loop {
        let mut vertices_to_remove: Vec<usize> = Vec::new();
        for v1 in graph.get_vertices() {
            if graph.get_vertex(*v1).nei_vert.len() != k - 1 {
                continue;
            }
            // if component.len() - visited_vert.len() < k {
            //     continue;
            // }
            let mut clique_candidates: HashSet<usize> = HashSet::with_capacity(k);
            clique_candidates.insert(*v1);
            for nei in &graph.get_vertex(*v1).nei_vert {
                clique_candidates.insert(*nei);
            }
            let mut is_clique = true;
            for nei in &graph.get_vertex(*v1).nei_vert {
                clique_candidates.remove(&nei);
                if !clique_candidates.is_subset(&graph.get_vertex(*nei).nei_vert) {
                    is_clique = false;
                    break;
                }
                clique_candidates.insert(*nei);
            }
            if is_clique {
                return Some(clique_candidates);
            } else {
                vertices_to_remove.push(*v1);
            }
        }
        if vertices_to_remove.len() == 0 {
            break;
        }
        for v in vertices_to_remove {
            graph.remove_vertex(v);
        }
        if graph.get_vertices().len() < k {
            break;
        }
    }

    return None;
}


#[allow(dead_code)]
const ADDRESS_BITS_PER_WORD: u16 = 6;
#[allow(dead_code)]
const BITS_PER_WORD: u16 = 1 << ADDRESS_BITS_PER_WORD;
#[allow(dead_code)]
const WORD_MASK: u64 = 0xFFFFFFFFFFFFFFFF;
#[allow(dead_code)]
struct DBitset {
    words_in_use: usize,
    words: Vec<u64>,
}
#[allow(dead_code,unused_parens)]
impl DBitset {
    fn word_index(nbits: usize) -> usize {
        nbits >> ADDRESS_BITS_PER_WORD
    }
    fn new(nbits: usize) -> DBitset {
        let l = DBitset::word_index(nbits - 1) + 1;
        let mut w = Vec::with_capacity(l);
        w.resize(l, 0);
        DBitset {
            words_in_use: 0,
            words: w,
        }
    }
    fn is_empty(&self) -> bool {
        self.words_in_use == 0
    }
    fn set(&mut self, bit_idx: usize) {
        let wordindex = DBitset::word_index(bit_idx);
        let mut bit = bit_idx;
        bit -= (wordindex << ADDRESS_BITS_PER_WORD);
        self.expand_to(wordindex);
        self.words[wordindex] |= (1u64 << bit);
    }
    fn setc(&mut self, bit_idx: usize) -> bool {
        let wordindex = DBitset::word_index(bit_idx);
        let mut bit = bit_idx;
        bit -= (wordindex << ADDRESS_BITS_PER_WORD);
        self.expand_to(wordindex);
        let w = self.words[wordindex];
        self.words[wordindex] |= (1u64 << bit);
        return w != self.words[wordindex];
    }
    fn reset(&mut self, bit_idx: usize) {
        let wordindex = DBitset::word_index(bit_idx);
        if wordindex >= self.words_in_use {
            return;
        }
        let mut bit = bit_idx;
        bit -= (wordindex << ADDRESS_BITS_PER_WORD);

        self.words[wordindex] &= !(1u64 << bit);
        self.recalculate_words_in_use();
    }

    fn get(&self, bit_idx: usize) -> bool {
        let word_index = DBitset::word_index(bit_idx);
        let mut bit = bit_idx;
        bit -= word_index << ADDRESS_BITS_PER_WORD;
        (word_index < self.words_in_use) && ((self.words[word_index] & (1u64 << bit)) != 0)
    }
    fn expand_to(&mut self, word_idx: usize) {
        let words_required = word_idx + 1;
        if self.words_in_use < words_required {
            self.words_in_use = words_required;
        }
        if self.words.len() < words_required {
            self.words.resize(words_required, 0);
        }
    }

    fn recalculate_words_in_use(&mut self) {
        self.words_in_use = 0;
        for i in (0..self.words.len()).rev() {
            if self.words[i] != 0 {
                self.words_in_use = i + 1;
                break;
            }
        }
    }

    fn and_with(&mut self, set: &DBitset) {
        let mut word_len = self.words_in_use;
        if self.words_in_use > set.words_in_use {
            word_len = set.words_in_use;
            for i in word_len..self.words_in_use {
                self.words[i] = 0;
            }
        }

        for i in 0..word_len {
            self.words[i] &= set.words[i];
        }
        self.recalculate_words_in_use();
    }
    fn and_not_with(&mut self, set: &DBitset) {
        let w_min = cmp::min(self.words_in_use, set.words_in_use);
        for i in 0..w_min {
            self.words[i] &= !set.words[i];
        }
        self.recalculate_words_in_use();
    }
    fn is_subset_of(&self, set: &DBitset) -> bool {
        if self.words_in_use > set.words_in_use {
            return false;
        }
        for i in 0..self.words_in_use {
            if (self.words[i] & (!set.words[i])) != 0 {
                return false;
            }
        }
        return true;
    }
    fn or_with(&mut self, set: &DBitset) -> bool {
        let mut changed = false;
        if self.words_in_use < set.words_in_use {
            self.words_in_use = set.words_in_use;
        }
        if self.words.len() < self.words_in_use {
            self.words.resize(self.words_in_use, 0);
        }
        let w_min = cmp::min(self.words_in_use, set.words_in_use);
        for i in 0..w_min {
            let w = self.words[i];
            self.words[i] |= set.words[i];
            if w != self.words[i] {
                changed = true;
            }
        }
        return changed;
    }

    fn least_significant_bit_position(m: u64) -> Option<usize> {
        let mut n = m;
        if n == 0 {
            return None;
        }

        let mut pos = 63usize;
        if n & 0x00000000FFFFFFFFu64 != 0 {
            pos -= 32;
        } else {
            n >>= 32;
        }
        if n & 0x000000000000FFFFu64 != 0 {
            pos -= 16;
        } else {
            n >>= 16;
        }
        if n & 0x00000000000000FFu64 != 0 {
            pos -= 8;
        } else {
            n >>= 8;
        }
        if n & 0x000000000000000Fu64 != 0 {
            pos -= 4;
        } else {
            n >>= 4;
        }
        if n & 0x0000000000000003u64 != 0 {
            pos -= 2;
        } else {
            n >>= 2;
        }
        if n & 0x0000000000000001u64 != 0 {
            pos -= 1;
        }
        return Some(pos);
    }

    fn next_set_bit(&self, from_index: usize) -> Option<usize> {
        let mut from_idx = from_index;
        let mut u = DBitset::word_index(from_idx);
        if u >= self.words_in_use {
            return None;
        }
        from_idx -= (u << ADDRESS_BITS_PER_WORD);
        let mut word = self.words[u] & (WORD_MASK << from_idx);
        while word == 0 {
            u += 1;
            if u >= self.words_in_use {
                return None;
            }
            word = self.words[u];
        }
        let bit = u << ADDRESS_BITS_PER_WORD;
        let lbit = DBitset::least_significant_bit_position(word);

        if bit == 0 && lbit.is_none() {
            return None;
        }

        return Some(bit + lbit.unwrap());
    }
}

fn make_dbitset_from(graph: &Graph) -> Vec<DBitset> {
    let vert_len = graph.vertices_pool.len();
    let mut res: Vec<DBitset> = Vec::with_capacity(vert_len);

    for _ in 0 ..  vert_len {
        res.push(DBitset::new(vert_len));
    }

    for v in graph.get_vertices() {
        let vert = graph.get_vertex(*v);
        let dv = &mut res[*v];
        dv.set(*v);
        for nei in &vert.nei_vert {
            dv.set(*nei);
        }
    }

    return res;
}


fn find_clique_bitset(graph: &mut Vec<DBitset>, vert_map: &mut HashMap<usize, usize>, k: usize) -> Option<HashSet<usize>> {
    loop {
        let mut vertex_to_remove: Option<usize> = None;
        for (v1, v1_len) in vert_map.iter() {
            if *v1_len != k - 1 {
                continue;
            }
            // println!("v {}", v1);
            let clique_candidates = &graph[*v1];
            let mut is_clique = true;
            let mut bit = clique_candidates.next_set_bit(0);
            while bit.is_some() {
                let nei = bit.unwrap();
                if nei == *v1 {
                    bit = clique_candidates.next_set_bit(nei + 1);
                    continue;
                }

                if !clique_candidates.is_subset_of(&graph[nei]) {
                    is_clique = false;
                    break;
                }
                bit = clique_candidates.next_set_bit(nei + 1);
            }

            if is_clique {
                // println!("is clique");
                let mut clique: HashSet<usize> = HashSet::new();
                bit = clique_candidates.next_set_bit(0);
                while bit.is_some() {
                    let nei = bit.unwrap();
                    clique.insert(nei);
                    bit = clique_candidates.next_set_bit(nei + 1);
                }
                return Some(clique);
            } 
            
            vertex_to_remove=Some(*v1);
            break;
        }
        match vertex_to_remove {
            Some(v) => {
                let mut vertices_to_update: Vec<usize> = Vec::with_capacity(graph.len());
                let clique_candidates = &graph[v];
                let mut bit = clique_candidates.next_set_bit(0);
                while bit.is_some() {
                    let nei = bit.unwrap();
                    vertices_to_update.push(nei);
                    
                    let vert_to_update = vert_map.get_mut(&v);
                    match vert_to_update {
                        Some(vert) => {
                            *vert -= 1;
                        }, 
                        None => {
                        }
                    }

                    bit = clique_candidates.next_set_bit(nei + 1);
                }
                vert_map.remove(&v);

                for nei in &vertices_to_update {
                    graph[*nei].reset(v);
                }
            },
            None => {
                break;
            }
        }
        // println!("before {:?}", vert_map);

        // println!("after {:?}", vert_map);

        if vert_map.len() < k {
            break;
        }
    }

    return None;
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
        // println!("start remove ");

        remove_vertices_min_nei(&mut graph, k - 1);

        if graph.get_num_vertices() == 0 {
            writeln!(output, "-1").expect("correct output");
            continue;
        }

        // let connected_components = graph.get_components();

        // let mut clique: Option<&HashSet<usize>> = None;

        // for component in &connected_components {
        //     // println!("component.len() {}", component.len());
        //     if component.len() == k {
        //         clique = Some(component);
        //         break;
        //     }
        // }
        // let mut clique_graph = graph.clone();
        let mut clique_graph = make_dbitset_from(&graph);
        let mut vert_map: HashMap<usize, usize> = HashMap::with_capacity(graph.vertices_pool.len());
        for v in graph.get_vertices() {
            vert_map.insert(*v, graph.get_vertex(*v).nei_vert.len());
        }

        // let inverted_graph = build_inverted_graph(&graph);

        remove_vertices_min_nei(&mut graph, k);

        if graph.get_num_vertices() > 0 {
            writeln!(output, "1 {}", graph.get_num_vertices()).expect("correct output");
            let collected_vertices: Vec<String> =  graph.get_vertices().iter().map(|&v| graph.get_vertex(v).data.to_string()).collect();
            // collected_vertices.sort();
            writeln!(output, "{}", collected_vertices.join(" ")).expect("correct output");
        } else {
            let component_clique = find_clique_bitset(&mut clique_graph, &mut vert_map, k);

            match component_clique {
                Some(vertices) => {
                    writeln!(output, "2").expect("correct output");
                    let collected_vertices: Vec<String> =  vertices.iter().map(|&v| graph.get_vertex(v).data.to_string()).collect();
                    // collected_vertices.sort();
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
1 2 3 5
1 10
1 10 2 3 4 5 6 7 8 9
-1
"
        );
    }
}
