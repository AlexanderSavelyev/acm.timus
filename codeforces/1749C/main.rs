use std::io::{self, BufReader};
use std::io::prelude::*;

fn check_alice(a: &Vec<usize>, k: usize) ->bool {
    if k == 0 {
        return true;
    }
    if k == 1 {
        return a[0] == 1;
    }
    let start = k - 1;
    let end = k * 2 - 1;
    let mut step: usize = 1;
    for idx in start..end {
        if a[idx] > step {
            return false;
        }
        step += 1;
    }
    return true;
}
fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut buf_str = String::new();

    reader.read_line(&mut buf_str).unwrap();
    let t: usize = buf_str.trim().parse().unwrap();

    let mut a: Vec<usize> = Vec::new();
    for _ in 0..t {
        a.clear();
        buf_str.clear();
        reader.read_line(&mut buf_str).unwrap();
        let n: f32 =  buf_str.trim().parse().unwrap();

        buf_str.clear();
        reader.read_line(&mut buf_str).unwrap();
        for a_str in buf_str.trim().split(" ") {
            let ai: usize = a_str.parse().expect("correct number");
            a.push(ai);
        }
        a.sort();
        // println!("{:?}", a);
        let mut k: usize = (n / 2.0 + 0.5) as usize; 

        // println!("{}", k);

        loop {
            if check_alice(&a, k) {
                break;
            }
            /*
            * Verify 1 by 1 but can be k log k
            */ 
            k -= 1;
        }

        // println!("{} {}", a,b);
        writeln!(output, "{}", k).expect("valid string");
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
        let test = String::from("4
3
1 1 2
4
4 4 4 4
1
1
5
1 3 2 1 1");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "2
0
1
3
");
    }
}
