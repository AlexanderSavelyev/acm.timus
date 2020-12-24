use std::io::{self, BufReader};
use std::io::prelude::*;

#[derive(Debug)]
struct Node {
    range_left: usize,
    range_right: usize,
    range_max: usize,
    range_sum: usize,
    nei1: usize,
    nei2: usize,
}

struct SegmentTree {
    nodes_pool: Vec<Node>,
    root_node: usize,
}

impl Node {
    fn new() -> Node {
        Node {
            range_left: 0,
            range_right: 0,
            range_max: 0,
            range_sum: 0,
            nei1: 0,
            nei2: 0, 
        }
    }
}

impl SegmentTree {
    fn new() -> SegmentTree {
        return SegmentTree{
            nodes_pool: Vec::new(),
            root_node: 0,
        };
    }

    fn build_from(&mut self, values: &Vec<usize>) {
        let mut stack: Vec<usize> = Vec::new();
        self.nodes_pool.push(Node::new());
        for &v in values {
            stack.push(self.nodes_pool.len());
            self.nodes_pool.push(Node{
                range_left: v,
                range_right: v,
                range_max: v,
                range_sum: v,
                nei1: 0,
                nei2: 0, 
            });
        }
        let mut stack_next: Vec<usize> = Vec::new();
        let mut next_node: Node = Node::new();
        // println!("{:?}", next_node);
        // let mut counter: usize = 0;
        loop {
            for &nei in &stack {
                let nei_node = &self.nodes_pool[nei];
                // println!("nei_node {:?}", nei_node);
                if next_node.nei1 == 0 {
                    next_node.nei1 = nei;
                    next_node.range_left = nei_node.range_left;
                    if nei_node.range_max > next_node.range_max {
                        next_node.range_max = nei_node.range_max;
                    }
                    next_node.range_sum = next_node.range_sum + nei_node.range_sum;
                } else if next_node.nei2 == 0 {
                    next_node.nei2 = nei;
                    next_node.range_right = nei_node.range_right;
                    if nei_node.range_max > next_node.range_max {
                        next_node.range_max = nei_node.range_max;
                    }
                    next_node.range_sum = next_node.range_sum + nei_node.range_sum;
                    println!("next_node {:?}", next_node);
                    stack_next.push(self.nodes_pool.len());
                    self.nodes_pool.push(next_node);
                    next_node = Node::new();
                }
            }
            if next_node.nei1 > 0 {
                stack_next.push(self.nodes_pool.len());
                self.nodes_pool.push(next_node);
                next_node = Node::new();
            }
            if stack_next.len() == 1 {
                self.root_node = stack_next[0];
                break;
            }
            stack.resize(stack_next.len(), 0_usize);
            stack.copy_from_slice(stack_next.as_slice());
            stack_next.clear();
            // counter += 1;
            // if counter > 10 {
            //     break;
            // }
        }
        
    }
}
fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let mut s = input.trim().split(' ');

    let n: usize = s.next().unwrap().trim().parse().unwrap();
    let q: usize = s.next().unwrap().trim().parse().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();

    let mut prices: Vec<usize> = Vec::with_capacity(n);
    let p = input.trim().split(' ');

    for a in p {
        prices.push(a.parse().unwrap());
    }

    println!("{:?}", prices);

    let mut segment_tree = SegmentTree::new();

    segment_tree.build_from(&prices);

    for _ in 0..q {
        input.clear();
        reader.read_line(&mut input).unwrap();
        // println!("{:?}", input);
        let mut s1 = input.trim().split(' ');

        let t: usize = s1.next().unwrap().trim().parse().unwrap();
        let x: usize = s1.next().unwrap().trim().parse().unwrap();
        let y: usize = s1.next().unwrap().trim().parse().unwrap();

        println!("{} {} {}", t, x, y);

        if t == 1 {

        } else {
            
        }

    }

    // println!("{}", n);
    writeln!(output, "1").expect("correct output");

}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use solve;

    #[test]
    fn basic_test() {
        let test = String::from("10 6
10 10 10 6 6 5 5 5 3 1
2 3 50
2 4 10
1 3 10
2 2 36
1 4 7
2 2 17");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "1
");
    }
}
