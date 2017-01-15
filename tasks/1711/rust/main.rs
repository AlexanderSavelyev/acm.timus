use std::io::{self, BufReader};
use std::io::prelude::*;


fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut lines: Vec<String> = Vec::new();
    let mut words: Vec<Vec<String>> = Vec::new();
    for _ in 0..n {
        let mut next_line = String::new();
        reader.read_line(&mut next_line).unwrap();
        lines.push(next_line);
        // println!("{:?}", input);


        // let mut s = input.trim().split(' ');

        // let a_str = s.next().unwrap();
        // let a: i32 = a_str.trim().parse().unwrap();

        // let b_str = s.next().unwrap();
        // let b: i32 = b_str.trim().parse().unwrap();

        // println!("{} {}", a,b);
    }
    input.clear();
    reader.read_line(&mut input).unwrap();
    let order = input.trim().split(' ');
    for o in order {
        let mut next: Vec<String> = Vec::new();
        let b: usize = o.trim().parse().unwrap();
        let next_input = &lines[b - 1];
        let n_spl = next_input.trim().split(' ');
        for ns in n_spl {
            next.push(ns.to_string());
        }
        words.push(next);
    }
    println!("{:?}", words);
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
        let test = String::from("11
cipher grille kamkohob
names codenames codes
newtests rejudge timus
size volume summit
watchmen braineater twosides
solution random yesorno
keywords subversion commands
bosses shooting shaitan
game strategy playgame
mnemonic palindromes bestname
eligibility rectangle rules
2 1 7 10 9 6 11 3 8 4 5");
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
