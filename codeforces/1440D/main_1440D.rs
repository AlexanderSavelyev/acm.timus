// extern crate rand;
use std::collections::{BTreeSet, HashSet};
use std::io::prelude::*;
use std::io::{self, BufReader};
// use std::collections::hash_set::Iter;
// use std::cmp;
// use std::time::{Duration, Instant};
// use rand::Rng;
// use rand::prelude::*;



#[allow(dead_code)]
const ADDRESS_BITS_PER_WORD: u16 = 6;
#[allow(dead_code)]
const BITS_PER_WORD: u16 = 1 << ADDRESS_BITS_PER_WORD;
#[allow(dead_code)]
const WORD_MASK: u64 = 0xFFFFFFFFFFFFFFFF;

#[allow(dead_code)]
#[derive(Debug)]
struct SparseMap {
    num_bits: u8,
    position: u32,
    reference: u32,
}

#[allow(dead_code)]
struct SparseBitset {
    words_map: Vec<SparseMap>,
    words: Vec<u64>,
    num_bits: usize,
}
#[allow(dead_code)]
impl SparseMap {
    fn new(num_bits: u8, position: u32, reference: u32) -> SparseMap {
        SparseMap {
            num_bits: num_bits,
            position: position,
            reference: reference,
        }
    }
}
#[allow(dead_code,unused_parens)]
impl SparseBitset {
    
    fn new(nbits: usize) -> SparseBitset {
        let last_word = SparseBitset::word_index(nbits - 1);
        let mut words_map = Vec::new();
        words_map.push(SparseMap::new(0, 0, last_word as u32));
        SparseBitset {
            words_map: words_map,
            words: Vec::new(),
            num_bits: 0,
        }
    }

    fn word_index(nbits: usize) -> usize {
        nbits >> ADDRESS_BITS_PER_WORD
    }

    // fn is_empty(&self) -> bool {
    //     self.words_in_use == 0
    // }
    fn word_map_contains(&self, mid: usize, word_idx: u32) -> bool {
        if self.words_map[mid].num_bits > 0 {
            return self.words_map[mid].position == word_idx;
        } else {
            return self.words_map[mid].position <= word_idx && word_idx <= self.words_map[mid].reference;
        }
    }
    fn find_map_idx(&self, word_idx: u32) -> usize {
        let mut size = self.words_map.len();
        let mut base = 0_usize;
        // println!("find word idx {}", word_idx);

        while size >= 1 {
            // mid: [base..size)
            let half = size / 2;
            let mid = base + half;
            if self.word_map_contains(mid, word_idx) {
                return mid;
            } 
            if self.words_map[mid].position < word_idx  {
                base = mid
            }
            size -= half;
        }
        panic!("incorrect state");
    }
    fn split_words(&mut self, map_idx: usize, word_idx: u32) -> usize {
        let old_position = self.words_map[map_idx].position;
        let old_reference = self.words_map[map_idx].reference;
        let ref_idx = self.words.len() as u32;
        self.words.push(0);
        if old_position == word_idx && old_reference == word_idx {
            self.words_map[map_idx].reference = ref_idx;
        } else {
            let new_map = SparseMap::new(0, word_idx, ref_idx);

            if old_position == word_idx {
                self.words_map[map_idx].position += 1;
                self.words_map.insert(map_idx, new_map);
            } else if old_reference == word_idx {
                self.words_map[map_idx].reference -= 1;
                self.words_map.insert(map_idx + 1, new_map);
                return map_idx + 1;
            } else {
                self.words_map.insert(map_idx, SparseMap::new(0, old_position, word_idx - 1));
                self.words_map[map_idx + 1].position = word_idx + 1;
                self.words_map.insert(map_idx + 1, new_map);
                return map_idx + 1;
            }
            
        }
        return map_idx;
    }

    fn merge_words(&mut self, map_idx: usize, word_idx: u32) {
        if map_idx > 0 && map_idx < self.words_map.len() - 1 {
            let next = map_idx + 1;
            let prev = map_idx - 1;
            if self.words_map[prev].num_bits == 0 && self.words_map[next].num_bits == 0 {
                self.words_map[prev].reference = self.words_map[next].reference;
                self.words_map.remove(map_idx);
                self.words_map.remove(map_idx);
            } else if self.words_map[prev].num_bits == 0 && self.words_map[next].num_bits > 0 {
                self.words_map[prev].reference = word_idx;
                self.words_map.remove(map_idx);
            } else if self.words_map[prev].num_bits > 0 && self.words_map[next].num_bits == 0 {
                self.words_map[next].position = word_idx;
                self.words_map.remove(map_idx);
            } else {
                self.words_map[map_idx].reference = word_idx;
            }
        } else if map_idx > 0 {
            let prev = map_idx - 1;
            if self.words_map[prev].num_bits == 0 {
                self.words_map[prev].reference = word_idx;
                self.words_map.remove(map_idx);
            } else {
                self.words_map[map_idx].reference = word_idx;
            }
        } else if map_idx < self.words_map.len() - 1 {
            let next = map_idx + 1;
            if self.words_map[next].num_bits == 0 {
                self.words_map[next].position = word_idx;
                self.words_map.remove(map_idx);
            } else {
                self.words_map[map_idx].reference = word_idx;
            }
        } else {
            self.words_map[map_idx].reference = word_idx;
        }
    }
    fn set(&mut self, bit_idx: usize) -> bool {
        // println!("set {}", bit_idx);
        let word_idx = SparseBitset::word_index(bit_idx);
        let mut map_idx = self.find_map_idx(word_idx as u32);
        if self.words_map[map_idx].num_bits == 0 {
            map_idx = self.split_words(map_idx, word_idx as u32);
        }
        let mut bit = bit_idx;
        bit -= (word_idx << ADDRESS_BITS_PER_WORD);
        let word = self.words_map[map_idx].reference as usize;
        let w = self.words[word];
        self.words[word] |= (1u64 << bit);
        if w != self.words[word] {
            self.words_map[map_idx].num_bits += 1;
            self.num_bits += 1;
            return true;
        }
        return false;
    }

    fn reset(&mut self, bit_idx: usize) -> bool {
        // println!("reset {}", bit_idx);
        let word_idx = SparseBitset::word_index(bit_idx);
        let map_idx = self.find_map_idx(word_idx as u32);

        if self.words_map[map_idx].num_bits == 0 {
            return false;
        }

        let mut bit = bit_idx;
        bit -= (word_idx << ADDRESS_BITS_PER_WORD);
        let word = self.words_map[map_idx].reference as usize;
        let w = self.words[word];
        self.words[word] &= !(1u64 << bit);
        if w != self.words[word] {
            self.words_map[map_idx].num_bits -= 1;
            self.num_bits -= 1;
            if self.words_map[map_idx].num_bits == 0 {
                self.merge_words(map_idx, word_idx as u32);
            }
            return true;
        }

        return false;
    }

    fn get(&self, bit_idx: usize) -> bool {
        let word_idx = SparseBitset::word_index(bit_idx);
        let map_idx = self.find_map_idx(word_idx as u32);

        if self.words_map[map_idx].num_bits == 0 {
            return false;
        }
        let mut bit = bit_idx;
        bit -= word_idx << ADDRESS_BITS_PER_WORD;
        let word = self.words_map[map_idx].reference as usize;

        (self.words[word] & (1u64 << bit)) != 0
    }

    // fn and_with(&mut self, set: &SparseBitset) {
    //     let mut word_len = self.words_in_use;
    //     if self.words_in_use > set.words_in_use {
    //         word_len = set.words_in_use;
    //         for i in word_len..self.words_in_use {
    //             self.words[i] = 0;
    //         }
    //     }

    //     for i in 0..word_len {
    //         self.words[i] &= set.words[i];
    //     }
    //     self.recalculate_words_in_use();
    // }
    // fn and_not_with(&mut self, set: &SparseBitset) {
    //     let w_min = cmp::min(self.words_in_use, set.words_in_use);
    //     for i in 0..w_min {
    //         self.words[i] &= !set.words[i];
    //     }
    //     self.recalculate_words_in_use();
    // }
    fn is_subset_of(&self, set: &SparseBitset) -> bool {
        let mut map_idx = 0_usize;
        for wm in &self.words_map {
            if wm.num_bits == 0 {
                continue;
            }
            let word_idx = wm.position;
            // println!("start {}", word_idx);
            loop {
                let set_wm = &set.words_map[map_idx];
                if set_wm.position <= word_idx {
                    if set_wm.num_bits == 0 && word_idx <= set_wm.reference {
                        // println!("return false num bits 0 {}", word_idx);
                        return false;
                    }
                    if set_wm.position == word_idx && set_wm.num_bits > 0{
                        // println!("check set {}", word_idx);
                        if (self.words[wm.reference as usize] & (!set.words[set_wm.reference as usize])) != 0 {
                            // println!("return false & {}", word_idx);
                            return false;
                        }
                        break;
                    }
                }
                map_idx += 1;
                if map_idx >= set.words_map.len() {
                    // println!("return false len {}", map_idx);
                    return false;
                }
            }
        }
        return true;
    }
    // fn or_with(&mut self, set: &SparseBitset) -> bool {
    //     let mut changed = false;
    //     if self.words_in_use < set.words_in_use {
    //         self.words_in_use = set.words_in_use;
    //     }
    //     if self.words.len() < self.words_in_use {
    //         self.words.resize(self.words_in_use, 0);
    //     }
    //     let w_min = cmp::min(self.words_in_use, set.words_in_use);
    //     for i in 0..w_min {
    //         let w = self.words[i];
    //         self.words[i] |= set.words[i];
    //         if w != self.words[i] {
    //             changed = true;
    //         }
    //     }
    //     return changed;
    // }

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

    fn next_set_bit(&self, from_index: usize, from_map: usize) -> Option<(usize, usize)> {
        let mut from_idx = from_index;
        let mut map_idx = from_map;
        let from_word = SparseBitset::word_index(from_idx);
        let mut word = 0_u64;
        loop {
            if map_idx >= self.words_map.len() {
                return None;
            }
            let wm = &self.words_map[map_idx];
            if wm.num_bits > 0 {
                if wm.position == (from_word as u32) {
                    from_idx -= (from_word << ADDRESS_BITS_PER_WORD);
                    word = self.words[wm.reference as usize] & (WORD_MASK << from_idx);
                } else {
                    if (map_idx == from_map) {
                        map_idx += 1;
                        continue;
                    }
                    word = self.words[wm.reference as usize];
                }
            }

            if word != 0 {
                let bit = (wm.position as usize) << ADDRESS_BITS_PER_WORD;
                let lbit = SparseBitset::least_significant_bit_position(word);
                
                return Some((bit + lbit.unwrap(), map_idx));
            }

            map_idx += 1;
        }
    }

    // let mut bit = cell.next_set_bit(0);
    // while bit.is_some() {
    //     let b = bit.unwrap();
    //     bit = cell.next_set_bit(b + 1);
    // }
    //
}




impl<'a> IntoIterator for &'a SparseBitset {
    type Item = usize;
    type IntoIter = SBitsetIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SBitsetIterator {
            bitset: self,
            index: None,
            map_idx: 0,
        }
    }
}

struct SBitsetIterator<'a> {
    bitset: &'a SparseBitset,
    index: Option<usize>,
    map_idx: usize,
}

impl<'a> Iterator for SBitsetIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let start_idx = match self.index {
            Some(idx) => {
                idx + 1
            }, 
            None => {
                0
            }
        };
        match self.bitset.next_set_bit(start_idx, self.map_idx) {
            Some((bit, from_map)) => {
                self.index.replace(bit);
                self.map_idx = from_map;
                return Some(bit);
            },
            None => {
                return None;
            }
        }
    }
}





struct BitsetGraph {
    vec_matrix: Vec<SparseBitset>,
    vertices: BTreeSet<usize>,
}

impl BitsetGraph {
    fn new(num_vertices: usize) -> BitsetGraph {
        let mut vec_matrix = Vec::with_capacity(num_vertices);
        let mut vertices = BTreeSet::new();
        for v in 0..num_vertices {
            let mut new_v = SparseBitset::new(num_vertices);
            new_v.set(v);
            vec_matrix.push(new_v);
            vertices.insert(v);
        }

        return BitsetGraph {
            vec_matrix: vec_matrix,
            vertices: vertices,
        }
    }

    fn add_edge(&mut self, v1: usize, v2: usize) {
        self.vec_matrix[v1].set(v2);
        self.vec_matrix[v2].set(v1);
    }

    fn remove_vertex(&mut self, v: usize, cache: &mut Vec<usize>) {
        cache.clear();
        for nei in &self.vec_matrix[v] {
            cache.push(nei);
            // println!("nei {}", nei);
        }

        for nei in cache {
            self.vec_matrix[*nei].reset(v);
            // println!("reset {}", nei);
        }
        self.vertices.remove(&v);
    }

}

fn remove_vertices_min_nei(graph: &mut BitsetGraph, min_nei: usize) {
    let mut vertices_queue: HashSet<usize> = HashSet::new();
    let mut vec_cache: Vec<usize> = Vec::new();

    for i in &graph.vertices {
        vertices_queue.insert(*i);
    }

    loop {
        let next_v = vertices_queue.iter().next().cloned();
        match next_v {
            Some(next_idx) => {
                // println!("check {}", next_idx);
                vertices_queue.remove(&next_idx);
                let next_vertex = &graph.vec_matrix[next_idx];
                // if next_vertex.num_bits == 0 {
                //     continue;
                // }
                // println!("next_vertex.num_bits {}", next_vertex.num_bits);
                if next_vertex.num_bits < min_nei + 1 {
                    for nei in next_vertex {
                        if nei == next_idx {
                            continue;
                        }
                        vertices_queue.insert(nei);
                        // println!("insert {}", nei);
                    }
                    // println!("remove {}", next_idx);
                    graph.remove_vertex(next_idx, &mut vec_cache);
                    // println!("remove done {}", next_idx);
                }
            },
            None => break
        }
    }
}



fn find_result(graph: &mut BitsetGraph, 
                        clique: &mut Option<BTreeSet<usize>>, 
                        result_set: &mut Option<BTreeSet<usize>>,
                        k: usize) {
    let mut vertex_to_remove: Vec<usize> = Vec::new();
    let mut vertices_to_update: Vec<usize> = Vec::new();

    loop {
        if graph.vertices.len() < k {
            break;
        }
        vertex_to_remove.clear();

        for &v in &graph.vertices {
            let vert_len = graph.vec_matrix[v].num_bits - 1;
            if vert_len < k - 1 {
                vertex_to_remove.push(v);
                continue;
            } else if vert_len >= k {
                continue;
            }
            if clique.is_some() {
                vertex_to_remove.push(v);
                continue;
            }
            let clique_candidates = &graph.vec_matrix[v];
            let mut is_clique = true;
            for nei in clique_candidates {
                if nei == v {
                    continue;
                }

                if !clique_candidates.is_subset_of(&graph.vec_matrix[nei]) {
                    is_clique = false;
                    break;
                }
            }
            if is_clique {
                // println!("is clique");
                let mut clique_c: BTreeSet<usize> = BTreeSet::new();
                for nei in clique_candidates {
                    clique_c.insert(nei);
                }
                clique.replace(clique_c);
            }
            vertex_to_remove.push(v);
        }

        // println!("vertex_to_remove {:?}", vertex_to_remove);
        
        if vertex_to_remove.len() == 0 {
            if graph.vertices.len() > k {
                result_set.replace(graph.vertices.clone());
            }
            break;
        }

        for &v in &vertex_to_remove {
            graph.remove_vertex(v, &mut vertices_to_update);
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

        // println!("test case {} {} {}", n, m, k);

        let mut graph: BitsetGraph = BitsetGraph::new(n);

        for _ in 0..m {
            input.clear();
            reader.read_line(&mut input).unwrap();
            let mut v = input.trim().split(' ');

            let v1: usize = v.next().unwrap().trim().parse().unwrap();
            let v2: usize = v.next().unwrap().trim().parse().unwrap();

            // println!("{} {}", v1, v2);

            graph.add_edge(v1 - 1, v2 - 1);

        }
        // println!("start remove ");

        remove_vertices_min_nei(&mut graph, k - 1);

        if graph.vertices.len() == 0 {
            writeln!(output, "-1").expect("correct output");
            continue;
        }

        let mut clique: Option<BTreeSet<usize>> = None;
        let mut result_set: Option<BTreeSet<usize>> = None;

        find_result(&mut graph, &mut clique, &mut result_set, k);

        match result_set {
            Some(vertices) => {
                writeln!(output, "1 {}", vertices.len()).expect("correct output");
                let collected_vertices: Vec<String> =  vertices.iter().map(|&v| (v + 1).to_string()).collect();
//              // collected_vertices.sort();
                writeln!(output, "{}", collected_vertices.join(" ")).expect("correct output");
            },
            None => {
                match clique {
                    Some(vertices) => {
                        writeln!(output, "2").expect("correct output");
                        let collected_vertices: Vec<String> =  vertices.iter().map(|&v| (v + 1).to_string()).collect();
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
}

fn main() {
    
    // let start = Instant::now();
    // solve(&mut io::stdin(), &mut io::stdout());
    // test_huge_graph();
    
    // let duration = start.elapsed();
    
    // println!("\nTotal time = {:?}", duration);
    solve(&mut io::stdin(), &mut io::stdout());
}

// fn test_huge_graph() {
//     let n = 10000;
//     let m = 15000;
//     let k = 20;
//     let mut graph: BitsetGraph = BitsetGraph::new(n);
//     let mut output = io::stdout();
//     let mut rng = rand::thread_rng();
//     let mut y: f64 = rng.gen();

//     for i in 0 .. m {
//         y= rng.gen();
//         let v1 = (y * n as f64) as usize;
//         y = rng.gen();
//         let v2 = (y * n as f64) as usize;

//         graph.add_edge(v1, v2);
//     }

//     remove_vertices_min_nei(&mut graph, k - 1);

//     if graph.vertices.len() == 0 {
//         writeln!(output, "-1").expect("correct output");
//         return;
//     }

//     let mut clique: Option<BTreeSet<usize>> = None;
//     let mut result_set: Option<BTreeSet<usize>> = None;

//     find_result(&mut graph, &mut clique, &mut result_set, k);

//     match result_set {
//         Some(vertices) => {
//             writeln!(output, "1 {}", vertices.len()).expect("correct output");
//             let collected_vertices: Vec<String> =  vertices.iter().map(|&v| (v + 1).to_string()).collect();
// //              // collected_vertices.sort();
//             // writeln!(output, "{}", collected_vertices.join(" ")).expect("correct output");
//         },
//         None => {
//             match clique {
//                 Some(vertices) => {
//                     writeln!(output, "2").expect("correct output");
//                     let collected_vertices: Vec<String> =  vertices.iter().map(|&v| (v + 1).to_string()).collect();
//                     // collected_vertices.sort();
//                     // writeln!(output, "{}", collected_vertices.join(" ")).expect("correct output");
//                 },
//                 None => {
//                     writeln!(output, "-1").expect("correct output");
//                 }
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use solve;

    #[test]
    fn basic_test1() {
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
1 2 3 4
1 10
1 2 3 4 5 6 7 8 9 10
-1
"
        );
    }
    #[test]
    fn basic_test3() {
        let test = String::from(
            "2
            6 7 3
            1 2
            1 3
            2 3
            3 4
            4 5
            3 6
            6 5
            2 1 2
            1 2"
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
1 2 3
2
1 2
"
        );
    }

    #[test]
    fn basic_test5() {
        let test = String::from(
            "7
            7 5 2
            3 5
            1 3
            2 6
            1 4
            1 2
            3 1 1
            2 3
            5 7 5
            1 5
            1 2
            2 3
            2 4
            1 3
            3 5
            3 4
            6 11 4
            1 3
            1 2
            1 5
            5 6
            3 5
            4 5
            1 6
            2 6
            1 4
            4 6
            3 4
            12 22 6
            2 4
            3 5
            10 11
            1 6
            4 5
            3 10
            7 9
            4 12
            2 8
            3 12
            9 11
            2 9
            11 12
            1 11
            3 9
            2 12
            1 2
            2 10
            10 12
            5 8
            4 10
            7 12
            8 12 5
            5 6
            6 8
            1 6
            3 8
            6 7
            3 4
            3 5
            2 5
            1 2
            1 3
            2 7
            4 6
            2 1 2
            1 2",
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
1 4
1 2
2 3
-1
2
1 3 4 5
-1
-1
2
1 2
"
        );
    }
    
}
