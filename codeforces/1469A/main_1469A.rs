use std::io::{self, BufReader};
use std::io::prelude::*;


fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut num_total = 0_usize;
        let mut idx_open = 0_usize;
        let mut idx_close = 0_usize;
        for a in input.trim().chars() {
            // println!("{:?}", a);
            if a == '(' {
                idx_open = num_total;
            } else if a == ')' {
                idx_close = num_total;
            }
            num_total += 1;
        }

        // println!("{} {} {}", num_total, idx_open, idx_close);

        if num_total % 2 == 1 || idx_close == 0 || idx_open == num_total - 1 {
            writeln!(output, "NO").expect("correct output");
            continue;
        }

        writeln!(output, "YES").expect("correct output");

        
        // let mut s = input.trim().split(' ');

        // let a_str = s.next().unwrap();
        // let a: i32 = a_str.trim().parse().unwrap();

        // let b_str = s.next().unwrap();
        // let b: i32 = b_str.trim().parse().unwrap();

        // println!("{} {}", a,b);
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
        let test = String::from("5
()
(?)
(??)
??()
)?(?");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "YES
NO
YES
YES
NO
");
    }
}
