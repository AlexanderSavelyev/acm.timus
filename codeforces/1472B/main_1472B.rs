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
        let n: usize = input.trim().parse().unwrap();

        input.clear();
        reader.read_line(&mut input).unwrap();
        
        let s = input.trim().split(' ');
        let mut num_ones = 0_usize;
        let mut num_twos = 0_usize;
        for ai in s {
            let a: usize = ai.parse().unwrap();
            if a == 1 {
                num_ones += 1;
            } else {
                num_twos += 1;
            }
        }

        if num_ones % 2 == 1 {
            writeln!(output, "NO").expect("valid string");
            continue;
        }
        if num_twos % 2 == 1 && num_ones < 2 {
            writeln!(output, "NO").expect("valid string");
            continue;
        }

        writeln!(output, "YES").expect("valid string");

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
2
1 1
2
1 2
4
1 2 1 2
3
2 2 2
3
2 1 2");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "YES
NO
YES
NO
NO
");
    }
}
