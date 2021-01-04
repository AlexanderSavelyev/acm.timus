use std::io::{self, BufReader};
use std::io::prelude::*;

const MAX_SIZE: usize = 200001;

fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    let mut a: Vec<usize> = vec![0; MAX_SIZE];
    let mut visited: Vec<u8> = vec![0; MAX_SIZE];

    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();
        // println!("{:?}", input);
        let n: usize = input.trim().parse().unwrap();

        input.clear();
        reader.read_line(&mut input).unwrap();
        
        let s = input.trim().split(' ');
        let mut a_idx = 1;
        for ai in s {
            let next_a: usize = ai.parse().unwrap();
            a[a_idx] = next_a;
            visited[a_idx] = 0;
            a_idx += 1;
        }
        let mut max_sum = 0_usize;
        let mut next_idx;
        let mut next_sum;
        for i in 1..=n {
            next_idx = i;
            if visited[next_idx] == 1 {
                continue;
            }
            next_sum = 0_usize;

            while next_idx <= n {
                next_sum += a[next_idx];
                visited[next_idx] = 1;
                next_idx += a[next_idx];
            }
            if next_sum > max_sum {
                max_sum = next_sum;
            }
        }

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
    fn basic_test() {
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
