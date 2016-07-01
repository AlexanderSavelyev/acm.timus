

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::process::Command;

const ADDRESS_BITS_PER_WORD: i32 = 6;
const BITS_PER_WORD: i32 = 1 << ADDRESS_BITS_PER_WORD;
const WORD_MASK: u64 = 0xFFFFFFFFFFFFFFFF;

struct DBitset {
    words_in_use: usize,
    bits_number: usize,
    length: usize,
    words: Vec<u64>,
}

impl DBitset {
    fn wordIndex(nbits: usize) -> usize {
        nbits >> ADDRESS_BITS_PER_WORD
    }
    fn new(nbits: usize) -> DBitset {
        let l = DBitset::wordIndex(nbits - 1) + 1;
        let mut w = Vec::with_capacity(l);
        for _ in 0..l {
            w.push(0);
        }

        DBitset {
            words_in_use: 0,
            bits_number: nbits,
            length: l,
            words: w,
        }
    }
    fn set(&mut self, bit_idx: usize) {
        let wordindex = DBitset::wordIndex(bit_idx);
        let mut bit = bit_idx;
        bit -= (wordindex << ADDRESS_BITS_PER_WORD);
        self.expand_to(wordindex);
        self.words[wordindex] |= (1u64 << bit);
    }

    fn get(&self, bit_idx: usize) -> bool {
        let word_index = DBitset::wordIndex(bit_idx);
        let mut bit = bit_idx;
        bit -= (word_index << ADDRESS_BITS_PER_WORD);
        (word_index < self.words_in_use) && ((self.words[word_index] & (1u64 << bit)) != 0)
    }
    fn expand_to(&mut self, word_idx: usize) {
        let words_required = word_idx + 1;
        if self.words_in_use < words_required {
            self.words_in_use = words_required;
        }
    }
}



fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::with_capacity(20, input);
    let mut input = String::new();

    let stack_num = 1000;
    let op_num = 100000;
    let part_num = 65000;


    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut st_values: Vec<u32> = Vec::with_capacity(op_num + 1);

    let mut st_nodes1: Vec<u16> = Vec::with_capacity(part_num);
    let mut st_nodes2: Vec<u16> = Vec::with_capacity(op_num - part_num + 1);
    let mut second_part = DBitset::new(op_num);

    let mut last_idx: Vec<usize> = Vec::with_capacity(stack_num);

    for _ in 0..stack_num {
        last_idx.push(0);
    }

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        // println!("{:?}", input);
        let mut v_in = input.trim().split(' ');
        let op = v_in.next().unwrap();

        if op.starts_with("PU") {
            let st_id: u32 = v_in.next().unwrap().parse().unwrap();
            let st_val: u32 = v_in.next().unwrap().parse().unwrap();

            let st_idx = st_id as usize - 1;

            // println!("{:?}, {}", st_id, st_val);

            st_values.push(st_val);
            let cur_idx = st_values.len();


            let cur_node = last_idx[st_idx];
            let node_part = cur_node % part_num;
            if cur_idx < st_nodes1.len() {
                st_nodes1.push(node_part as u16);

            } else {
                st_nodes2.push(node_part as u16);
            }
            if node_part < cur_node {
                second_part.set(st_idx);
            }
            //st_nodes.push(cur_node);

            last_idx[st_idx] = cur_idx;

        } else {
            let st_id: u32 = v_in.next().unwrap().parse().unwrap();
            let st_idx = st_id as usize - 1;

            let cur_idx = last_idx[st_idx];
            let st_val = st_values[cur_idx - 1];
            // println!("{:?}", st_val);
            writeln!(output, "{}", st_val).expect("correct output");
            if second_part.get(cur_idx - 1) {
                //last_idx[st_idx] = st_nodes2[(cur_idx - 1) % part_num];
            } else {
                //last_idx[st_idx] = st_nodes1[cur_idx - 1] as usize;
            }
            //last_idx[st_idx] = st_nodes[cur_idx - 1];
        }

    }

    // writeln!(output, "{}", n).expect("correct output");
    // io::stdin().read_line(&mut input);
    show_mem();

}

fn show_mem() {
    let output = Command::new("sh")
                     .arg("-c")
                     .arg("ps -u | grep main | grep -v grep | grep -v build | awk '{print $6 - \
                           2048}'")
                     .output()
                     .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    println!("MEM = {} kb",
             String::from_utf8_lossy(&output.stdout).trim());
}

fn main() {
    // solve(&mut io::stdin(), &mut io::stdout());
    // let mut f = File::create("../huge.txt").expect("correct test");
    // let num = 100000;
    // writeln!(f, "{}", num);
    // for _ in 0..num-1 {
    //     writeln!(f, "PUSH 1 1000");
    // }
    // writeln!(f, "POP 1");
    let mut f = File::open("../huge.txt").expect("correct test");
    solve(&mut f, &mut io::stdout());



    // let mut input = String::new();
    // io::stdin().read_line(&mut input);
    // ps -u | grep main | grep -v grep | awk '{print $6 - 2048}'
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use solve;

    #[test]
    fn basic_test() {
        let mut f = File::open("../input.txt").expect("correct test");
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut f, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "400\n200\n300\n");
    }
}
