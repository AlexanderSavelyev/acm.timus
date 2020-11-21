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
        let mut numbers: Vec<usize> = Vec::new();

        let n_str = s.next().unwrap();
        let n: usize = n_str.trim().parse().unwrap();
 
        let k_str = s.next().unwrap();
        let k: usize = k_str.trim().parse().unwrap();

        input.clear();
        reader.read_line(&mut input).unwrap();

        let elements = input.trim().split(' ');
        println!("{} {}", n, k);

        for elem in elements {
            let e: usize = elem.parse().expect("correct number");
            numbers.push(e);
        }

        println!("{:?}", numbers);
        
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
        2 4
        0 24 34 58 62 64 69 78
        2 2
        27 61 81 91
        4 3
        2 4 16 18 21 27 36 53 82 91 92 95
        3 4
        3 11 12 22 33 35 38 67 69 71 94 99
        2 1
        11 41
        3 3
        1 1 1 1 1 1 1 1 1");
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
