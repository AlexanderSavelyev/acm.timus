use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::HashMap;

fn insert_dist(dist: i32, c: char, dist_map: &mut HashMap<char, i32>, diff: i32) {
    if dist <= (13 + diff) {
        dist_map.insert(c, (dist - diff).abs());
    } else {
        dist_map.insert(c, 26 + diff - dist);
    }
}

fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();
    let mut dist_a: HashMap<char, i32> = HashMap::new();
    let mut dist_c: HashMap<char, i32> = HashMap::new();
    let mut dist_t: HashMap<char, i32> = HashMap::new();
    let mut dist_g: HashMap<char, i32> = HashMap::new();

    for (i, c) in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().enumerate() {
        let dist = i as i32;
        insert_dist(dist, c, &mut dist_a, 0);
        insert_dist(dist, c, &mut dist_c, 2);
        if dist <= 6 {
            dist_t.insert(c, 7 + dist);
        } else {
            dist_t.insert(c, (dist - 19).abs());
        }
        insert_dist(dist, c, &mut dist_g, 6);
    }
    // for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
    //     println!("{} {}", c, dist_t[&c]);
    // }
    

    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();
    let s = input.trim();
    // println!("{:?}", s);

    let char_vec: Vec<char> = s.chars().collect();
    let mut min_sum = -1;
    for i in 0..n-3 {
        let sum = dist_a.get(&char_vec[i]).expect("has values") + 
        dist_c.get(&char_vec[i + 1]).expect("has values") + 
        dist_t.get(&char_vec[i + 2]).expect("has values") + 
        dist_g.get(&char_vec[i + 3]).expect("has values");

        if min_sum == -1 || sum < min_sum {
            min_sum = sum;
        }
    }

    // for c in s.chars() {
    //     println!("{:?}", c);
    // }

    writeln!(output, "{}", min_sum).expect("correct output");

}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use solve;

    #[test]
    fn basic_test1() {
        let test = String::from("4
        ZCTH");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "2\n");
    }
    #[test]
    fn basic_test2() {
        let test = String::from("5
        ZDATG");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "5\n");
    }
    #[test]
    fn basic_test3() {
        let test = String::from("6
        AFBAKC");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "16\n");
    }
}
