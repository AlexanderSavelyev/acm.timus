use std::io::{self, BufReader};
use std::io::prelude::*;
const DIVIDER: usize = 40;

fn calcuate_max(num_vec: &Vec<usize>) -> (usize, usize) {
    let mut max_num = 0_usize;
    let mut max_num2 = 0_usize;
    let mut max_num_idx = 0_usize;
    let mut max_num2_idx = 0_usize;
    for (i, v) in num_vec.iter().enumerate() {
        if *v >= max_num {
            if max_num > 0 {
                max_num2 = max_num;
                max_num2_idx = max_num_idx;
            }
            max_num = *v;
            max_num_idx = i;
        } else if *v >= max_num2 {
            max_num2 = *v;
            max_num2_idx = i;
        }

    }
    return (max_num_idx, max_num2_idx);
}

fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    let mut num_vec: Vec<usize> = Vec::new();
    let mut num_map: Vec<usize> = Vec::new();
    let mut num_taken: Vec<u8> =  vec![0; 200001];
    let mut num_steps: Vec<(usize, usize)> = Vec::new();

    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();
        
        let n: usize = input.trim().parse().unwrap();
        num_map.clear();
        num_vec.clear();
        num_steps.clear();

        
        let adduct = 
                if n % DIVIDER > 0 {
                    1
                } else {
                    0
                };
        let mut d = n / DIVIDER + adduct;
        num_map.push(n);
        while d > 2 {
            num_map.push(d);
            let adduct = 
                if d % DIVIDER > 0 {
                    1
                } else {
                    0
                };
            d = d / DIVIDER + adduct;
        }
        num_map.push(2);

        for &i in &num_map {
            num_taken[i] = 1;
            num_vec.push(i);
        }

        loop {
            let (max_num1_idx, max_num2_idx) = calcuate_max(&num_vec);
            let max_num1 = num_vec[max_num1_idx];
            let max_num2 = num_vec[max_num2_idx];
            // println!("max_num {} {}", max_num1, max_num2);
            if max_num1 == 2 && max_num2 == 1 {
                break;
            }
            num_steps.push((num_map[max_num1_idx], num_map[max_num2_idx]));
            let adduct = 
                if max_num1 % max_num2 > 0 {
                    1
                } else {
                    0
                };

            num_vec[max_num1_idx] =  max_num1 / max_num2 + adduct;
            // println!("{:?}", num_vec);
        }

        writeln!(output, "{}", n - 1 - num_map.len() + num_steps.len()).expect("correct output");
        // println!("{}", n - 1 - num_map.len() + num_steps.len());

        for i in 3..n {
            if num_taken[i] == 0 {
                writeln!(output, "{} {}", i, n).expect("correct output");
                // println!("{} {}", i, n);
            }
        }
        for &(m1, m2) in &num_steps {
            writeln!(output, "{} {}", m1, m2).expect("correct output");
            // println!("{} {}", m1, m2);
        }

        for &i in &num_map {
            num_taken[i] = 0;
        }
        // writeln!(output, "{}", n - 1).expect("correct output");

        // for i in (3..=n).rev() {
        //     // println!("{:?}", i);
        //     writeln!(output, "{} {}", i, i - 1).expect("correct output");
        // }
        // writeln!(output, "3 2").expect("correct output");



        // let mut s = input.trim().split(' ');

        // let a_str = s.next().unwrap();
        // let a: i32 = a_str.trim().parse().unwrap();

        // let b_str = s.next().unwrap();
        // let b: i32 = b_str.trim().parse().unwrap();

        // println!("{} {}", a,b);
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
        let test = String::from("29
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
        ");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
//         assert_eq!(res,
//                   "2
// 3 2
// 3 2
// 4
// 5 4
// 4 3
// 3 2
// 3 2
// ");
    }

    #[test]
    fn basic_test2() {
        let test = String::from("1
200000
");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
//         assert_eq!(res,
//                   "2
// 3 2
// 3 2
// 4
// 5 4
// 4 3
// 3 2
// 3 2
// ");
    }
}
