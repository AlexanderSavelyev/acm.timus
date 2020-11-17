use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::HashMap;

fn collect_step(steps: &mut Vec<Vec<usize>>, i: usize, j: usize, next_s: &[usize; 3]) {
    let mut s: Vec<usize> = Vec::new();
    for c in next_s {
        if *c == 1 {
            s.push(i + 1);
            s.push(j + 1);
        } else if * c == 2 {
            s.push(i + 1);
            s.push(j + 2);
        } else if * c == 3 {
            s.push(i + 2);
            s.push(j + 1);
        } else if * c == 4 {
            s.push(i + 2);
            s.push(j + 2);
        }
    }
    steps.push(s);
}
fn collect_steps(collected_steps: &mut Vec<Vec<usize>>,  steps: &HashMap<&str, ([usize; 3], &str)>, k: &str, i: usize, j: usize) {
    let mut next_k: &str = k;
    // println!("{:?}", k);

    while next_k != "0000" {
        let (next_s, next_step) = steps.get(next_k).expect("all correct");
        // println!("step {} -> {}", next_k, next_step);
        next_k = next_step;
        collect_step(collected_steps, i, j, next_s);
    }
}


fn get_key(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> String {
    let mut k = String::with_capacity(4);
    k.push(matrix[i][j]);
    k.push(matrix[i][j + 1]);
    k.push(matrix[i + 1][j]);
    k.push(matrix[i + 1][j + 1]);
    return k;
}
fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let t: usize = input.trim().parse().unwrap();

    let mut steps: HashMap<&str, ([usize; 3], &str)> = HashMap::new();

    steps.insert("1111", ([1, 2, 3], "0001"));
    steps.insert("1110", ([1, 2, 3], "0000"));
    steps.insert("1101", ([1, 2, 4], "0000"));
    steps.insert("1011", ([1, 3, 4], "0000"));
    steps.insert("0111", ([2, 3, 4], "0000"));
    steps.insert("1100", ([2, 3, 4], "1011"));
    steps.insert("0110", ([1, 3, 4], "1101"));
    steps.insert("0011", ([1, 2, 3], "1101"));
    steps.insert("1010", ([1, 2, 4], "0111"));
    steps.insert("1001", ([1, 2, 3], "0111"));
    steps.insert("0101", ([1, 2, 3], "1011"));
    steps.insert("1000", ([1, 2, 3], "0110"));
    steps.insert("0100", ([1, 2, 3], "1010"));
    steps.insert("0010", ([1, 2, 3], "1100"));
    steps.insert("0001", ([2, 3, 4], "0110"));

    for _ in 0..t {
        let mut matrix: Vec<Vec<char>> = Vec::new();
        let mut collected_steps: Vec<Vec<usize>> = Vec::new();
        input.clear();
        reader.read_line(&mut input).unwrap();
        // println!("{:?}", input);
        let mut s = input.trim().split(' ');

        let n_str = s.next().unwrap();
        let n: usize = n_str.trim().parse().unwrap();

        let m_str = s.next().unwrap();
        let m: usize = m_str.trim().parse().unwrap();

        // println!("{} {}", n, m);
       
        for _ in 0..n {
            input.clear();
            reader.read_line(&mut input).unwrap();
            let mut row: Vec<char> = Vec::new();
            for e in input.trim().chars() {
                row.push(e);
            }
            // println!("{:?}", row);
            matrix.push(row);
        }
        // writeln!(output, "{}", min_s).expect("correct output");

        for i in 0 .. n/2 {
            for j in 0 .. m/2 {
                // println!("{} {}", i, j);
                let k = get_key(&matrix, i * 2, j * 2);
                collect_steps(&mut collected_steps, &steps, &k.as_str(), i * 2, j * 2);
            }
        }

        if m % 2 == 1 {
            let last_column = m - 1;
            for i in 0 .. n/2 {
                matrix[i * 2][last_column - 1] = '0';
                matrix[i * 2 + 1][last_column - 1] = '0';

                let k = get_key(&matrix, i * 2, last_column - 1);
                collect_steps(&mut collected_steps, &steps, &k.as_str(), i * 2, last_column - 1);
            } 
        }
        if n % 2 == 1 {
            let last_row = n - 1;
            for j in 0 .. m/2 {
                matrix[last_row - 1][j * 2] = '0';
                matrix[last_row - 1][j * 2 + 1] = '0';

                let k = get_key(&matrix, last_row - 1, j * 2);
                collect_steps(&mut collected_steps, &steps, &k.as_str(), last_row - 1, j * 2);
            } 
        }
        if n % 2 == 1 && m % 2 == 1 {
            let last_column = m - 1;
            let last_row = n - 1;
            matrix[last_row - 1][last_column - 1] = '0';
            matrix[last_row - 1][last_column] = '0';
            matrix[last_row][last_column - 1] = '0';
            let k = get_key(&matrix, last_row - 1, last_column - 1);
            collect_steps(&mut collected_steps, &steps, &k.as_str(), last_row - 1, last_column - 1);
        }

        writeln!(output, "{}", collected_steps.len()).expect("correct output");
        for step in &collected_steps {
            writeln!(output, "{} {} {} {} {} {}", step[0], step[1], step[2], step[3], step[4], step[5]).expect("correct output");
        }
    }
    // writeln!(output, "{}", min_s).expect("correct output");


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
        2 2
        10
        11
        3 3
        011
        101
        110
        4 4
        1111
        0110
        0110
        1111
        5 5
        01011
        11001
        00010
        11011
        10000
        2 3
        011
        101");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
//         assert_eq!(res,
//                   "3
// 52
// 5
// 10
// 16
// 22
// ");
    }
}
