use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(PartialEq, Eq, Debug, Clone)]
struct Data {
    a: i64,
    b: i64,
}

impl Data {
    fn new(a: i64, b: i64) -> Data {
        Data { a: a, b: b }
    }
}

fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap();

    let mut queue: BTreeMap<i64, Vec<Data>> = BTreeMap::new();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        // println!("{:?}", input);
        let mut s = input.trim().split(' ');

        let a_str = s.next().unwrap();
        let a: i64 = a_str.trim().parse().unwrap();

        let b_str = s.next().unwrap();
        let b: i64 = b_str.trim().parse().unwrap();
        let k = Data::new(a, b);
        let d = a - b;
        let v = queue.entry(d).or_insert(Vec::new());
        v.push(k);
        // println!("{} {}", a, b);
    }

    let mut current_pos: i64 = 0;
    let mut current_sum: i64 = 0;
    for (_, v) in queue.iter().rev() {
        for data in v {
            current_pos += 1;
            current_sum += (current_pos - 1) * data.a + (n - current_pos) * data.b;
            // println!("{} {}", data.a, data.b);
        }
    }
    // println!("sum = {}", current_sum);

    // println!("{}", n);
    writeln!(output, "{}", current_sum).expect("valid output");
}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    // use std::fs::File;
    use solve;

    #[test]
    fn basic_test_1() {
        let test = String::from(
            "3
            4 2
            2 3
            6 1",
        );
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "12\n");
    }
    #[test]
    fn basic_test_2() {
        let test = String::from(
            "4
            2 4
            3 3
            7 1
            2 3",
        );
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "25\n");
    }
    #[test]
    fn basic_test_3() {
        let test = String::from(
            "10
            5 10
            12 4
            31 45
            20 55
            30 17
            29 30
            41 32
            7 1
            5 5
            3 15",
        );
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "1423\n");
    }
}
