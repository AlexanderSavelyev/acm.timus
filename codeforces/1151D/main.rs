use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(PartialEq, Eq, Debug, Clone)]
struct UNode {
    a: u32,
    b: u32,
}

impl UNode {
    fn new(a: u32, b: u32) -> UNode {
        UNode { a: a, b: b }
    }
    fn insert_map(queue: &mut BTreeMap<UNode, u32>, k: UNode) {
        queue.insert(k.clone(),  1 + if queue.contains_key(&k) { queue[&k] } else { 0 });
    }
}

impl PartialOrd for UNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.a == other.a {
            return self.b.cmp(&other.b);
        } else {
            return other.a.cmp(&self.a);
        }
    }
}

fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    let mut left_queue: BTreeMap<UNode, u32> = BTreeMap::new();
    let mut right_queue: BTreeMap<UNode, u32> = BTreeMap::new();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        // println!("{:?}", input);
        let mut s = input.trim().split(' ');

        let a_str = s.next().unwrap();
        let a: u32 = a_str.trim().parse().unwrap();

        let b_str = s.next().unwrap();
        let b: u32 = b_str.trim().parse().unwrap();
        let k = UNode::new(a, b);
        if a > b {
            UNode::insert_map(&mut left_queue, k);
        } else {
            UNode::insert_map(&mut right_queue, k);
        }
        // println!("{} {}", a, b);
    }

    let mut ordered: Vec<UNode> = Vec::new();
    let mut current_pos = 0;
    let mut current_sum = 0;
    println!("left");
    for (k , v) in &left_queue {
        for _ in 0..*v {
            ordered.push(k.clone());
            current_pos += 1;
            current_sum += (current_pos - 1) * k.a + (n - current_pos) * k.b;
        }
        println!("{} {} size {}", k.a, k.b, v);
    }
    println!("right");
    for (k , v) in right_queue.iter().rev() {
        for _ in 0..*v {
            ordered.push(k.clone());
            current_pos += 1;
            current_sum += (current_pos - 1) * k.a + (n - current_pos) * k.b;
        }
        println!("{} {} size {}", k.a, k.b, v);
    }

    // println!("{}", n);
    writeln!(output, "{}", "test").expect("valid output");
}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    // use std::fs::File;
    use solve;
    use UNode;
    use std::collections::BTreeMap;

    #[test]
    fn basic_test_1() {
        let mut queue: BTreeMap<UNode, u32> = BTreeMap::new();
        UNode::insert_map(&mut queue, UNode::new(11, 10));
        UNode::insert_map(&mut queue, UNode::new(10, 20));
        UNode::insert_map(&mut queue, UNode::new(10, 10));
        UNode::insert_map(&mut queue, UNode::new(10, 10));

        for (k , v) in &queue {
            println!("{} {} size {}", k.a, k.b, v);
        }
    }

    #[test]
    fn basic_test_2() {
        let test = String::from(
            "3
4 2
2 3
6 1",
        );
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "test\n");
        //assert_eq!(res,
        //                   "2297.0716
        //936297014.1164
        //0.0000
        //37.7757
        //");
    }
}
