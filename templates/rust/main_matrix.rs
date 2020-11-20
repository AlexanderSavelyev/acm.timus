use std::io::{self, BufReader};
use std::io::prelude::*;

fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let mut matrix: Vec<Vec<usize>> = Vec::new();

    input.clear();
    reader.read_line(&mut input).unwrap();
    // println!("{:?}", input);
    let mut s = input.trim().split(' ');

    let n_str = s.next().unwrap();
    let n: usize = n_str.trim().parse().unwrap();

    let m_str = s.next().unwrap();
    let m: usize = m_str.trim().parse().unwrap();

    // println!("{} {}", n, m);
    
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut row: Vec<usize> = Vec::new();
        for e in input.trim().split(" ") {
            row.push(e.parse().expect("correct number"));
        }
        // println!("{:?}", row);
        matrix.push(row);
    }
    writeln!(output, " ").expect("correct output");

    // writeln!(output, "{}", min_s).expect("correct output");


}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use solve;

    #[test]
    fn basic_test() {
        let test = String::from("3 2
        0 0
        0 0
        0 0");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
//         assert_eq!(res,
//                   "3
// 52
// 5
// 10
// 16
// 22
// ");
    }
}
