use std::io::{self, BufReader};
use std::io::prelude::*;

const MAX_SIZE: usize = 200001;

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

        let mut odd_a: Vec<usize> = Vec::new();
        let mut even_a: Vec<usize> = Vec::new();
        
        let s = input.trim().split(' ');
        odd_a.push(0);
        even_a.push(0);
        for ai in s {
            let next_a: usize = ai.parse().unwrap();
            if next_a % 2 == 1 {
                odd_a.push(next_a);
            } else {
                even_a.push(next_a);
            }
        }

        let mut alice_num = 0_usize;
        let mut bob_num = 0_usize;

        let mut odd_idx = odd_a.len() - 1;
        let mut even_idx = even_a.len() - 1;

        odd_a.sort();
        even_a.sort();

        loop {
            if even_idx == 0 && odd_idx == 0 {
                break;
            }
            // Alice
            if even_idx > 0 {
                if odd_idx > 0 {
                    if even_a[even_idx] > odd_a[odd_idx] {
                        alice_num += even_a[even_idx];
                        even_idx -= 1;
                    } else {
                        odd_idx -= 1;
                    }
                } else {
                    alice_num += even_a[even_idx];
                    even_idx -= 1;
                }
            } else if odd_idx > 0 {
                odd_idx -= 1;
            }
            // Bob
            if odd_idx > 0 {
                if even_idx > 0 {
                    if odd_a[odd_idx] > even_a[even_idx] {
                        bob_num += odd_a[odd_idx];
                        odd_idx -= 1;
                    } else {
                        even_idx -= 1;
                    }
                } else {
                    bob_num += odd_a[odd_idx];
                    odd_idx -= 1;
                }
            } else if even_idx > 0 {
                even_idx -= 1;
            }
        }

        if alice_num == bob_num {
            writeln!(output, "Tie").expect("valid string");
        } else if alice_num > bob_num {
            writeln!(output, "Alice").expect("valid string");
        } else {
            writeln!(output, "Bob").expect("valid string");
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
        let test = String::from("4
4
5 2 7 3
3
3 2 1
4
2 2 2 2
2
7 8");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "Bob
Tie
Alice
Alice
");
    }
}
