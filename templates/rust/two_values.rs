use std::io::{self, BufReader};
use std::io::prelude::*;


fn read_two_values(reader: &mut dyn io::BufRead, str_buf: &mut String) -> (usize, usize) {
    str_buf.clear();
    reader.read_line(str_buf).unwrap();
    // println!("{:?}", input);
    let mut s = str_buf.trim().split(' ');

    let n: usize = s.next().unwrap().trim().parse().unwrap();
    let m: usize = s.next().unwrap().trim().parse().unwrap();
    return (n, m)
}

fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut str_buf = String::new();

    reader.read_line(&mut str_buf).unwrap();
    let t: usize = str_buf.trim().parse().unwrap();

    for _ in 0..t {
        let (n, m) = read_two_values(&mut reader, &mut str_buf);
        for _ in 0..m {
            let (_, _) = read_two_values(&mut reader, &mut str_buf);
        }
        if n > m {
            writeln!(output, "YES").expect("valid string");
        } else {
            writeln!(output, "NO").expect("valid string");
        }

        // println!("{} {}", a,b);
    }

    // println!("{}", n);


    fn main() {
        solve(&mut io::stdin(), &mut io::stdout());
    }
    
    #[cfg(test)]
    mod tests {
        // use std::fs::File;
        use solve;
    
        #[test]
        fn basic_test() {
            let test = String::from("2
    2 2
    1 2
    2 1
    3 1
    2 2");
            //let mut f = File::open("../input.txt").expect("correct test");
            let testb = test.into_bytes();
            let mut test_r = testb.as_slice();
            let mut buf: Vec<u8> = Vec::new();
            solve(&mut test_r, &mut buf);
    
            let res = String::from_utf8(buf).expect("valid string");
            assert_eq!(res,
                      "NO
    YES
    ");
        }
    }
