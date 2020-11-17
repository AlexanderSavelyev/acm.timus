use std::io::{self, BufReader};
use std::io::prelude::*;
 
 
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
 
        let n_str = s.next().unwrap();
        let n: i32 = n_str.trim().parse().unwrap();
 
        let c0_str = s.next().unwrap();
        let c0: i32 = c0_str.trim().parse().unwrap();
 
        let c1_str = s.next().unwrap();
        let c1: i32 = c1_str.trim().parse().unwrap();
 
        let h_str = s.next().unwrap();
        let h: i32 = h_str.trim().parse().unwrap();
 
        // println!("{} {} {} {}", n,c0, c1, h);
 
        input.clear();
        reader.read_line(&mut input).unwrap();
 
        let bs = input.trim();
        // println!("{}", bs);
        let mut min_s: i32 = 0;
        let mut num_0 = 0;
        let mut num_1 = 0;
        for c in bs.chars() {
            // println!("{}", c);
            if c =='0' {
                min_s += c0;
                num_0 += 1;
            } else if c == '1' {
                min_s += c1;
                num_1 += 1;
            }
        }
 
        if c0 == c1 {
            writeln!(output, "{}", min_s).expect("correct output");
            continue;
        } else if h >= c0 && h >= c1 {
            writeln!(output, "{}", min_s).expect("correct output");
            continue;
        } else {
            if c0 < c1 && (h + c0) < c1 {
                min_s -= num_1 * (c1 - h - c0);
            } else if c1 < c0 && (h + c1) < c0 {
                min_s -= num_0 * (c0 - h - c1);
            }
        }
        writeln!(output, "{}", min_s).expect("correct output");
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
        let test = String::from("6
        3 1 1 1
        100
        5 10 100 1
        01010
        5 10 1 1
        11111
        5 1 10 1
        11111
        12 2 1 10
        101110110101
        2 100 1 10
        00");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);
 
        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "3
52
5
10
16
22
");
    }
}