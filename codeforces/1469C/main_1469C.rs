use std::io::{self, BufReader};
use std::io::prelude::*;
use std::cmp;



fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();

        // println!("{:?}", input);
        let mut s = input.trim().split(' ');

        let n: usize =  s.next().unwrap().trim().parse().unwrap();
        let k: i64 =  s.next().unwrap().trim().parse().unwrap();

        input.clear();
        reader.read_line(&mut input).unwrap();
        // hei.clear();
        let mut hei_idx = 0_usize;
        let mut has_no = false;
        let mut min_h: i64 = 0;
        let mut max_h: i64 = 0;
        for h in input.trim().split(' ') {
            let next_hei:i64 = h.trim().parse().unwrap();
            if hei_idx == 0 {
                min_h = next_hei;
                max_h = next_hei;
            } else {
                min_h = cmp::max(min_h - k + 1, next_hei);
                max_h = cmp::min(max_h + k - 1, next_hei + k - 1);
            }
            if min_h > max_h {
                has_no = true;
                break
            }
            if hei_idx == n - 1 {
                if next_hei < min_h || next_hei > max_h {
                    has_no = true;
                }
            }
            hei_idx += 1;
        }

        if has_no {
            writeln!(output, "NO").expect("correct output");
        } else {
            writeln!(output, "YES").expect("correct output");
        }
        
    }

}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    // use std::fs::File;
    use solve;

    #[test]
    fn basic_test1() {
        let test = String::from("3
6 3
0 0 2 5 1 1
2 3
0 2
3 2
3 0 2");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "YES
YES
NO
");
    }


    #[test]
    fn basic_test2() {
        let test = String::from("10
20 5
9 9 7 10 7 10 10 2 8 3 6 0 1 3 7 1 9 5 1 1
20 3
7 7 5 3 2 2 7 7 7 10 5 0 8 1 5 8 6 6 8 1
20 2
3 10 8 6 3 7 5 4 10 4 9 5 6 5 4 0 8 1 2 1
20 4
9 1 10 9 9 5 2 2 10 5 3 9 4 7 0 1 10 10 4 9
20 3
5 6 6 10 9 10 6 0 6 3 3 6 0 3 3 2 5 8 7 7
20 2
2 6 8 3 10 0 7 1 10 5 0 9 7 1 0 3 4 9 6 6
20 6
9 0 2 9 8 4 4 8 0 3 9 3 5 1 9 1 2 9 6 3
20 5
0 1 9 10 2 2 7 2 10 7 2 0 8 0 9 8 1 0 4 4
20 6
7 3 1 9 9 3 4 5 2 9 4 8 8 3 1 4 7 3 4 7
20 4
3 1 1 2 5 7 10 8 6 0 2 8 2 8 0 4 6 9 3 7
        ");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "YES
NO
NO
NO
NO
NO
YES
NO
YES
NO
");
    }

    #[test]
    fn basic_test3() {
        let test = String::from("1
20 5
0 1 9 10 2 2 7 2 10 7 2 0 8 0 9 8 1 0 4 4");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "NO
");
    }

    
}
