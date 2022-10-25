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
        println!("{:?}", buf_str);
        // let mut s = buf_str.trim().split(' ');

        // let a_str = s.next().unwrap();
        // let a: i32 = a_str.trim().parse().unwrap();

        // let b_str = s.next().unwrap();
        // let b: i32 = b_str.trim().parse().unwrap();

        // println!("{} {}", a,b);
        writeln!(output, "{} {}", a,b).expect("valid string");
    }

    // println!("{}", n);


}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use solve;

    #[test]
    fn basic_test() {
        let test = String::from("5
              ab
              acb
              bc
              abac
              babbc");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        //assert_eq!(res,
//                   "2297.0716
//936297014.1164
//0.0000
//37.7757
//");
    }
}
