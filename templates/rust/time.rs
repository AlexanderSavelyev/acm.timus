extern crate rand;
use std::time::{Duration, Instant};
use rand::Rng;
use rand::prelude::*;

fn main() {
    
    let start = Instant::now();
    test_huge_graph();
    
    let duration = start.elapsed();
    
    // println!("\nTotal time = {:?}", duration);
    // solve(&mut io::stdin(), &mut io::stdout());

    // pip install gprof2dot
    // install graphviz
    // valgrind --tool=callgrind target/debug/rust
    // gprof2dot -f callgrind callgrind.out. | dot -Tsvg -o output.svg
}

fn test_huge_graph() {
    let n = 10000;
    let m = 10000;
    let k = 23;
    // let mut graph: BitsetGraph = BitsetGraph::new(n);
    // let mut output = io::stdout();
    // let mut rng = rand::thread_rng();
    let mut y: f64 = rng.gen();

    for i in 0 .. m {
        y= rng.gen();
        let v1 = (y * n as f64) as usize;
        y = rng.gen();
        let v2 = (y * n as f64) as usize;

        // graph.add_edge(v1, v2);
    }
}