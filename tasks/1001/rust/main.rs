use std::io;
use std::io::prelude::*;


fn main() {
    let mut v: Vec<i64> = Vec::new();
    let stdin = io::stdin();

    for t in stdin.lock().lines() {
        let cur_line = t.unwrap();
        for u in cur_line.split(' ') {
            let cur_txt = u.trim();
            if cur_txt.len() > 0 {
                let cur_num: i64 = cur_txt.parse().unwrap();
                v.push(cur_num);
            }
        }
    }
    let mut i = v.len();
    while i > 0 {
        i -= 1;
        let f_num = v[i] as f64;
        println!("{:.*}", 4, f_num.sqrt());
    }
}
