use std::io::{self, BufReader};
use std::io::prelude::*;


fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    for _ in 1..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut s = input.trim().split(' ');

        let a_str = s.next().unwrap();
        let a: i32 = a_str.trim().parse().unwrap();

        let b_str = s.next().unwrap();
        let b: i32 = b_str.trim().parse().unwrap();

        println!("{} {}", a,b);
    }

    println!("{}", n);


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
        let mut f = File::open("../input.txt").expect("correct test");
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut f, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        //assert_eq!(res,
//                   "2297.0716
//936297014.1164
//0.0000
//37.7757
//");
    }
}
