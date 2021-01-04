use std::io::{self, BufReader};
use std::io::prelude::*;
use std::cmp;

const MAX_SIZE: usize = 200001;

fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    let mut a: Vec<usize> = vec![0; MAX_SIZE];
    let mut sums: Vec<usize> = vec![0; MAX_SIZE];

    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();
        // println!("{:?}", input);
        let n: usize = input.trim().parse().unwrap();

        input.clear();
        reader.read_line(&mut input).unwrap();

        let s = input.trim().split(' ');
        let mut a_idx = 0_usize;
        for ai in s {
            let next_a: usize = ai.parse().unwrap();
            a_idx += 1;
            a[a_idx] = next_a;
            sums[a_idx] = 0;
        }
        let mut max_sum = 0_usize;

        // for i in 1..=n {
            // println!("a {} {}", i, a[i]);
        // }

        for i in 1..=n {
            let sum = a[i] + sums[i];
            // println!("sum {}", sum);
            let next_idx = i + a[i];
            // println!("next_idx {}", next_idx);
            if next_idx > n {
                max_sum = cmp::max(max_sum, sum);
                // println!("max_sum {}", max_sum);
            } else {
                sums[next_idx] =  cmp::max(sums[next_idx], sum);
                // println!("put sum {} {}", next_idx, sum);
            }
        }
        // for i in 1..=n {
        //     println!("sums {} {}", i, sums[i]);
        // }
        writeln!(output, "{}", max_sum).expect("valid string");

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
    fn basic_test1() {
        let test = String::from("1
3
2 1 4");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "6
");
    }

    #[test]
    fn basic_test2() {
        let test = String::from("4
5
7 3 1 2 3
3
2 1 4
6
2 1000 2 3 995 1
5
1 1 1 1 1");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "7
6
1000
5
");
    }
}
