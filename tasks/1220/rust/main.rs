

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::process::Command;
use std::mem;

const ADDRESS_BITS_PER_WORD: u16 = 6;
const BITS_PER_WORD: u16 = 1 << ADDRESS_BITS_PER_WORD;
const WORD_MASK: u64 = 0xFFFFFFFFFFFFFFFF;

struct DBitset {
    words_in_use: usize,
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
    let part_num: usize = 65000;


    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut st_values: Vec<u32> = Vec::with_capacity(n + 1);

    let mut st_nodes: Vec<u16> = Vec::with_capacity(n + 1);
    let mut second_part = DBitset::new(n + 1);

    let mut last_idx: Vec<usize> = Vec::with_capacity(stack_num);

    for _ in 0..stack_num {
        last_idx.push(0);
    }
    let mut first_val: u32 = 0;
    
    let mut n_vals_16: usize = 0;


    // if n > 65000 {
    //     panic!("failed to execute process");
    // }
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        // println!("{:?}", input);
        let mut v_in = input.trim().split(' ');
        let op = v_in.next().unwrap();

        if op.starts_with("PU") {
            let st_id: u32 = v_in.next().unwrap().parse().unwrap();
            let st_val: u32 = v_in.next().unwrap().parse().unwrap();
            // if st_val > 65000 {
            //     n_vals_16 += 1;
            // }
            // if first_val == 0 {
            //     first_val = st_val;
            // }
            // if n == 100000 && n_vals_16 > 65000  {
            //     panic!("failed to execute process");
            // }

            let st_idx = st_id as usize - 1;

            // println!("{:?}, {}", st_id, st_val);

            st_values.push(st_val);
            let cur_idx = st_values.len();


            let cur_node = last_idx[st_idx];
            let node_part = cur_node % part_num;
            st_nodes.push(node_part as u16);
            if node_part < cur_node {
                second_part.set(cur_node);
            }
            // st_nodes.push(cur_node);

            last_idx[st_idx] = cur_idx;

        } else {
            let st_id: u32 = v_in.next().unwrap().parse().unwrap();
            let st_idx = st_id as usize - 1;

            let cur_idx = last_idx[st_idx];
            let st_val = st_values[cur_idx - 1];
            // println!("{:?}", st_val);
            writeln!(output, "{}", st_val).expect("correct output");
            output.flush();
            if second_part.get(cur_idx - 1) {
                last_idx[st_idx] = st_nodes[(cur_idx - 1) % part_num] as usize + part_num;
            } else {
                last_idx[st_idx] = st_nodes[cur_idx - 1] as usize;
            }
            // last_idx[st_idx] = st_nodes[cur_idx - 1];
        }
    }

    // if n == 100000 {
    //     panic!("failed to execute process");
    // }

    // writeln!(output, "{}", n).expect("correct output");
    // io::stdin().read_line(&mut input);
    //show_mem();
    // std::thread::sleep(std::time::Duration::from_millis(500));
    // println!("{} kb", show_mem() - 692);

}

fn solve2(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::with_capacity(20, input);
    let mut input = String::new();

    let stack_num = 1000;
    let part_num: usize = 65535;
    let part1_cap_size = 70000;
    let part2_cap_size = 40000;


    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut st_values1: Vec<u16> = Vec::with_capacity(part1_cap_size);
    let mut st_values2: Vec<u32> = Vec::with_capacity(part2_cap_size);

    let mut st_nodes1: Vec<u16> = Vec::with_capacity(part1_cap_size);
    let mut node_part1 = DBitset::new(part_num);

    let mut st_nodes2: Vec<u16> = Vec::with_capacity(part2_cap_size);
    let mut node_part2 = DBitset::new(part_num);


    let mut last_idx: Vec<u16> = Vec::with_capacity(stack_num);
    let mut last_part: Vec<bool> = Vec::with_capacity(stack_num);

    for _ in 0..stack_num {
        last_idx.push(0);
        last_part.push(false);
    }
    // let mut first_val: u32 = 0;
    
    // let mut n_vals_16: usize = 0;

    // if n > 65000 {
    //     panic!("failed to execute process");
    // }
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        // println!("{:?}", input);
        let mut v_in = input.trim().split(' ');
        let op = v_in.next().unwrap();

        if op.starts_with("PU") {
            // println!("{:?}", st_nodes1.capacity());
            let st_id: u16 = v_in.next().unwrap().parse().unwrap();
            let st_val: u32 = v_in.next().unwrap().parse().unwrap();
            // if st_val <= 65000 {
            //     n_vals_16 += 1;
            // }
            // if first_val == 0 {
            //     first_val = st_val;
            // }

            let st_idx = st_id as usize - 1;

            // println!("{:?}, {}", st_id, st_val);

            let cur_node = last_idx[st_idx];
            
            if st_val <= part_num as u32 {
                if last_part[st_idx] {
                    node_part1.set(st_nodes1.len());
                }
                st_nodes1.push(cur_node);

                st_values1.push(st_val as u16);
                let cur_idx = st_values1.len();
                last_idx[st_idx] = cur_idx as u16;
                last_part[st_idx] = false;
            } else {
                if last_part[st_idx] {
                    node_part2.set(st_nodes2.len());
                }
                st_nodes2.push(cur_node);

                st_values2.push(st_val);
                let cur_idx = st_values2.len();
                last_idx[st_idx] = cur_idx as u16;
                last_part[st_idx] = true;
            }
            
        } else {
            let st_id: u32 = v_in.next().unwrap().parse().unwrap();
            let st_idx = st_id as usize - 1;

            let cur_idx = last_idx[st_idx] as usize - 1;
            let st_val = if last_part[st_idx] {st_values2[cur_idx]} else {st_values1[cur_idx] as u32};
            // println!("{:?}", st_val);
            writeln!(output, "{}", st_val).expect("correct output");
            if last_part[st_idx] {
                last_idx[st_idx] = st_nodes2[cur_idx];
                last_part[st_idx] = node_part2.get(cur_idx);
            } else {
                last_idx[st_idx] = st_nodes1[cur_idx];
                last_part[st_idx] = node_part1.get(cur_idx);
            }

        }
    }



    // writeln!(output, "{}", n).expect("correct output");
    // io::stdin().read_line(&mut input);
    //show_mem();
    // std::thread::sleep(std::time::Duration::from_millis(500));
    // println!("{} kb", show_mem() - 692);

}


fn show_mem() -> usize{
    let output = Command::new("sh")
                     .arg("-c")
                     .arg("ps -u | grep main | grep -v grep | grep -v build | awk '{print $6 - \
                           2048}'")
                     .output()
                     .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    //println!("MEM = {} kb",
    //         String::from_utf8_lossy(&output.stdout).trim());

    String::from_utf8_lossy(&output.stdout).trim().parse().unwrap()
}

fn test_huge() {
    // let mut f = File::create("../huge2.txt").expect("correct test");
    // let num = 60000;
    // writeln!(f, "{}", num);
    // for _ in 0..num-3 {
    //     writeln!(f, "PUSH 1 1000");
    // }
    // writeln!(f, "PUSH 2 100000");
    // writeln!(f, "POP 1");
    // writeln!(f, "POP 2");

    show_mem();
    show_mem();
    show_mem();
    let av = show_mem();
    let avg = show_mem()-av;
    // println!("{:?}", avg);
    // std::thread::sleep(std::time::Duration::from_millis(1000));

    let start_mem = show_mem();
    // std::thread::sleep(std::time::Duration::from_millis(1000));
    println!("start mem {}", start_mem + avg);
    // let mut v1: Vec<u16> = Vec::with_capacity(op_num);
    // let mut v2: Vec<u32> = Vec::with_capacity(op_num);
    //let mut second_part = DBitset::new(op_num);
    let mut f = File::open("../huge2.txt").expect("correct test");
    solve2(&mut f, &mut io::stdout());
    // for _ in 0..op_num {
    //     v1.push(1000);
    //     v2.push(1000);
    // }
     // std::thread::sleep(std::time::Duration::from_millis(500));
     // println!("{} kb", (show_mem() - start_mem -avg));

    // let mut input = String::new();
    // io::stdin().read_line(&mut input);
    // ps -u | grep main | grep -v grep | awk '{print $6 - 2048}'
}

fn main() {
    solve2(&mut io::stdin(), &mut io::stdout());
    //test_huge();
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use solve2;

    #[test]
    fn basic_test() {
        let mut f = File::open("../input.txt").expect("correct test");
        let mut buf: Vec<u8> = Vec::new();
        solve2(&mut f, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "400\n200\n300\n");
    }
}
