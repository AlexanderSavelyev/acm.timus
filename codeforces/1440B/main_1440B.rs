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
        let mut numbers: Vec<u64> = Vec::new();

        let n_str = s.next().unwrap();
        let n: usize = n_str.trim().parse().unwrap();
 
        let k_str = s.next().unwrap();
        let k: usize = k_str.trim().parse().unwrap();

        input.clear();
        reader.read_line(&mut input).unwrap();

        let elements = input.trim().split(' ');
        // println!("{} {}", n, k);

        for elem in elements {
            let e: u64 = elem.parse().expect("correct number");
            numbers.push(e);
        }

        // println!("{:?}", numbers);

        let median = n - n / 2 - n % 2;
        // println!("median {}", median);
        let mut num_arrays: usize = 0;
        let mut rev_iter: usize = 0;
        let mut max_sum: u64 = 0;

        for i in (0..numbers.len()).rev() {
            if rev_iter >= median {
                max_sum += numbers[i];
                // println!("i {} number {}", i, numbers[i]);
                rev_iter = 0;
                num_arrays += 1;
                if num_arrays >= k {
                    break;
                }
                continue;
            }
            rev_iter += 1;
        }
        // println!("sum {}", max_sum);
        writeln!(output, "{}", max_sum).expect("correct output");

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
        assert_eq!(res,
                  "165
108
145
234
11
3
");
    }
}
