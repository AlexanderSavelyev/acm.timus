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

        let mut w: usize = s.next().unwrap().parse().unwrap();
        let mut h: usize = s.next().unwrap().parse().unwrap();
        let n: usize = s.next().unwrap().parse().unwrap();

        let mut num_parts = 1;

        while w % 2 == 0 {
            if num_parts >= n {
                break;
            }
            w >>= 1;
            num_parts <<= 1;
            
        }
        while h % 2 == 0 {
            if num_parts >= n {
                break;
            }
            h >>= 1;
            num_parts <<= 1;
        }

        if num_parts >= n {
            writeln!(output, "YES").expect("valid string");
        } else {
            writeln!(output, "NO").expect("valid string");
        }

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
        let test = String::from("5
2 2 3
3 3 2
5 10 2
11 13 1
1 4 4");
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
YES
");
    }
}
