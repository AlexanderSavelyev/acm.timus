use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::LinkedList;

#[derive(Debug)]
struct Node {
    range_left: usize,
    range_right: usize,
    range_min: usize,
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
            range_min: std::usize::MAX,
            range_sum: 0,
            nei1: 0,
            nei2: 0, 
        }
    }

    fn range_within(&self, elem: usize) -> bool {
        return elem >= self.range_left && elem <= self.range_right;
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
            let next_idx = self.nodes_pool.len();
            stack.push(next_idx);
            self.nodes_pool.push(Node{
                range_left: next_idx,
                range_right: next_idx,
                range_min: v,
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
                    next_node.range_right = nei_node.range_right;
                    if nei_node.range_min < next_node.range_min {
                        next_node.range_min = nei_node.range_min;
                    }
                    next_node.range_sum = next_node.range_sum + nei_node.range_sum;
                } else if next_node.nei2 == 0 {
                    next_node.nei2 = nei;
                    next_node.range_right = nei_node.range_right;
                    if nei_node.range_min < next_node.range_min {
                        next_node.range_min = nei_node.range_min;
                    }
                    next_node.range_sum = next_node.range_sum + nei_node.range_sum;
                    println!("next_node {} {:?}", self.nodes_pool.len(), next_node);
                    stack_next.push(self.nodes_pool.len());
                    self.nodes_pool.push(next_node);
                    next_node = Node::new();
                }
            }
            if next_node.nei1 > 0 {
                // println!("next_node {:?}", next_node);
                stack_next.push(next_node.nei1);
                // self.nodes_pool.push(next_node);
                next_node = Node::new();
            }
            // println!("stack_next.len() {:?}", stack_next.len());
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

    fn get_node(&self, idx: usize) -> &Node {
        return &self.nodes_pool[idx];
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
    let mut queue: LinkedList<usize> = LinkedList::new();
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
            queue.clear();
            queue.push_back(segment_tree.root_node);
            let mut money = y;
            let mut collected_shops = 0_usize;
            loop {
                println!("queue {:?}", queue);
                match queue.pop_back() {
                    Some(next_node) => {
                        let node = segment_tree.get_node(next_node);
                        if x <= node.range_right && node.range_min <= money {
                            println!("range_sum {} money {}", node.range_sum, money);
                            if x <= node.range_left && node.range_sum <= money {
                                money -= node.range_sum;
                                collected_shops += node.range_right - node.range_left + 1;
                            } else {
                                if node.nei2 > 0 {
                                    queue.push_back(node.nei2);
                                }
                                if node.nei1 > 0 {
                                    queue.push_back(node.nei1);
                                }
                            }
                        }
                        if money == 0 {
                            break;
                        }
                    }, 
                    None => {
                        break;
                    }
                }
            }
            println!("collected_shops {}", collected_shops);
            writeln!(output, "{}", collected_shops).expect("correct output");

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
//         assert_eq!(res,
//                   "1
// ");
    }
}
