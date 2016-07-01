

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::process::Command;



fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::with_capacity(20, input);
    let mut input = String::new();

    let stack_num = 1000;


    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut st_values: Vec<u32> = Vec::new();
    let mut st_nodes: Vec<usize> = Vec::new();

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
            st_nodes.push(cur_node);

            last_idx[st_idx] = cur_idx;

        } else {
            let st_id: u32 = v_in.next().unwrap().parse().unwrap();
            let st_idx = st_id as usize - 1;

            let cur_idx = last_idx[st_idx];
            let st_val = st_values[cur_idx - 1];
            // println!("{:?}", st_val);
            writeln!(output, "{}", st_val).expect("correct output");
            last_idx[st_idx] = st_nodes[cur_idx - 1];
        }

    }

    // writeln!(output, "{}", n).expect("correct output");
    //io::stdin().read_line(&mut input);
    show_mem();

}

fn show_mem() {
    let output = Command::new("sh")
                     .arg("-c")
                     .arg("ps -u | grep main | grep -v grep | grep -v build | awk '{print $6 - 2048}'")
                     .output()
                     .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    println!("MEM = {} kb", String::from_utf8_lossy(&output.stdout).trim());
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
