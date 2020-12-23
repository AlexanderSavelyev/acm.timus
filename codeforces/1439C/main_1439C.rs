use std::io::{self, BufReader};
use std::io::prelude::*;


fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let mut s = input.trim().split(' ');

    let n: usize = s.next().unwrap().trim().parse().unwrap();
    let q: usize = s.next().unwrap().trim().parse().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();

    let mut prices: Vec<usize> = Vec::new();
    let mut p = input.trim().split(' ');

    for a in p {
        prices.push(a);
    }

    for _ in 0..q {
        input.clear();
        reader.read_line(&mut input).unwrap();
        // println!("{:?}", input);
        let mut s1 = input.trim().split(' ');

        let t: usize = s1.next().unwrap().trim().parse().unwrap();
        let x: usize = s1.next().unwrap().trim().parse().unwrap();
        let y: usize = s1.next().unwrap().trim().parse().unwrap();
    }

    // println!("{}", n);

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
        //assert_eq!(res,
//                   "2297.0716
//936297014.1164
//0.0000
//37.7757
//");
    }
}
