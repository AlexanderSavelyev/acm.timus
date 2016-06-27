use std::io::{self, BufReader};
use std::io::prelude::*;

use std::collections::BTreeMap;

fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    for i in 1..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        println!("{:?}", input);
    }
    

    // writeln!(output, "{}", n).expect("correct output");

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
        //assert_eq!(res, "GAMES\n DRIVERS\nHOME\nWIN\n SOFT\nWINNT\n DRIVERS\n SYSTEM32\n  CERTSRV\n   CERTCO~1\n    X86\n  CONFIG\n");
    }
}
