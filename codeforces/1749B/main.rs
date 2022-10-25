use std::io::{self, BufReader};
use std::io::prelude::*;


fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut buf_str = String::new();

    reader.read_line(&mut buf_str).unwrap();
    let t: usize = buf_str.trim().parse().unwrap();

    for _ in 0..t {
        buf_str.clear();
        reader.read_line(&mut buf_str).unwrap();
        let _: usize =  buf_str.trim().parse().unwrap();
        let mut sum_a = 0usize;
        let mut sum_b = 0usize;
        let mut max_b = 0usize;

        buf_str.clear();
        reader.read_line(&mut buf_str).unwrap();
        for a_str in buf_str.trim().split(" ") {
            let a: usize = a_str.parse().expect("correct number");
            sum_a += a;
        }
        buf_str.clear();
        reader.read_line(&mut buf_str).unwrap();
        for b_str in buf_str.trim().split(" ") {
            let b: usize = b_str.parse().expect("correct number");
            sum_b += b;
            if max_b < b {
                max_b = b;
            }
        }

        // println!("{} {}", a,b);
        writeln!(output, "{}", (sum_a + sum_b - max_b)).expect("valid string");
    }

    // println!("{}", n);


}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    // use std::fs::File;
    use solve;

    #[test]
    fn basic_test() {
        let test = String::from("4
1
10
0
3
100 1 100
1 100 1
4
2 6 7 3
3 6 0 5
2
1000000000 1000000000
1000000000 1000000000");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "10
203
26
3000000000
");
    }
}
