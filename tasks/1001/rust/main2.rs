use std::io::{self, BufReader};
use std::io::prelude::*;


fn solve(input: &mut Read, output: &mut Write) {
    let reader = BufReader::new(input);
    let mut v: Vec<i64> = Vec::new();

    for t in reader.lines()
                   .map(|a| a.expect("correct input")) {
        for u in t.split(' ')
                  .map(|a| a.trim())
                  .filter(|a| a.len() > 0)
                  .map(|a| a.parse::<i64>().expect("parsed integer")) {
            v.push(u);
        }
    }

    for u in v.into_iter().rev().map(|a| a as f64) {
        writeln!(output, "{:.*}", 4, u.sqrt()).expect("valid output");
    }
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
        let mut f = File::open("test1.txt").expect("correct test");
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut f, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                   "2297.0716
936297014.1164
0.0000
37.7757
");
    }
}
