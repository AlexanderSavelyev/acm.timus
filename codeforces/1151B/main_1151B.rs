use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::HashMap;


fn next_conf(unique_iter: &mut Vec<usize>, unique: &Vec<Vec<usize>>) -> bool {
    let mut iter_idx = 0;

    while iter_idx < unique_iter.len() {
        if unique_iter[iter_idx] + 1 >= unique[iter_idx].len() {
            iter_idx += 1;
            continue;
        } else {
            unique_iter[iter_idx] += 1;
            if iter_idx > 0 {
                for prev_idx in 0 .. iter_idx {
                    unique_iter[prev_idx] = 0;
                }
            }
            return true;
        }
    }
    return false;
}

fn calculate_xor(matrix: &Vec<Vec<usize>>, unique: &Vec<Vec<usize>>, non_trivial_rows: &Vec<usize>, unique_iter: &Vec<usize>) -> usize{
    let mut result_xor = 0;
    for elem_idx in 0 .. unique_iter.len() {
        let elem = unique_iter[elem_idx];
        let elem_row = non_trivial_rows[elem_idx];
        let elem_column = unique[elem_idx][elem];
        let next_elem = matrix[elem_row][elem_column];
        result_xor ^= next_elem;
    }
    return result_xor;
}

fn update_seq(non_trivial_rows: &Vec<usize>, unique: &Vec<Vec<usize>>, unique_iter: &Vec<usize>, res_seq: &mut Vec<usize>) {
    for elem_idx in 0 .. unique_iter.len() {
        let elem = unique_iter[elem_idx];
        let elem_row = non_trivial_rows[elem_idx];
        let elem_column = unique[elem_idx][elem];
        res_seq[elem_row] = elem_column;
    }
}

fn solve(input: &mut dyn Read, output: &mut dyn Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    let mut matrix: Vec<Vec<usize>> = Vec::new();

    input.clear();
    reader.read_line(&mut input).unwrap();
    // println!("{:?}", input);
    let mut s = input.trim().split(' ');

    let n_str = s.next().unwrap();
    let n: usize = n_str.trim().parse().unwrap();

    let m_str = s.next().unwrap();
    let _: usize = m_str.trim().parse().unwrap();

    // println!("{} {}", n, m);
    let mut res_xor: usize = 0;
    let mut trivial_xor: usize = 0;
    let mut res_seq: Vec<usize> = Vec::new();
    let mut unique: Vec<Vec<usize>> = Vec::new();
    let mut non_trivial_rows: Vec<usize> = Vec::new();
    let mut unique_iter: Vec<usize> = Vec::new();
    
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut row: Vec<usize> = Vec::new();
        let mut unique_row: HashMap<usize, usize> = HashMap::new();
        // println!("{:?}", input.trim());
        for e in input.trim().split(" ") {
            let num: usize = e.parse().expect("correct number");
            if row.len() == 0 {
                res_xor ^= num;
                res_seq.push(0);
            }
            if !unique_row.contains_key(&num) {
                unique_row.insert(num, row.len());
            }
            
            row.push(num);
        }
        if unique_row.len() > 1 {
            let mut unique_row_vec: Vec<usize> = Vec::new();
            for (_,v) in unique_row {
                unique_row_vec.push(v);
            }
            unique_row_vec.sort();
            unique.push(unique_row_vec);
            non_trivial_rows.push(matrix.len());
            unique_iter.push(0);
        } else {
            trivial_xor ^= row[0];
        }
        // println!("{:?}", row);
        matrix.push(row);
    }
    // println!("unique {:?}", unique);
    /*
     * Calculate XOR for first row
     */
    let mut has_diff = false;
    if res_xor == 0 {
        while next_conf(&mut unique_iter, &unique) {
            res_xor = calculate_xor(&matrix, &unique, &non_trivial_rows, &unique_iter);
            // println!("calculate_xor {}", res_xor);
            res_xor ^= trivial_xor;
            if res_xor != 0 {
                update_seq(&non_trivial_rows, &unique, &unique_iter, &mut res_seq);
                has_diff = true;
                break;
            }
        }
    } else {
        has_diff = true;
    }

    if has_diff {
        writeln!(output, "TAK").expect("correct output");
        let joined: Vec<String> = res_seq.iter().map(|&j| (j + 1).to_string()).collect();
        writeln!(output, "{}", joined.join(" ")).expect("correct output");
    } else {
        writeln!(output, "NIE").expect("correct output");
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
    fn basic_test_1() {
        let test = String::from("3 2
        0 0
        0 0
        0 0");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "NIE\n");
    }
    #[test]
    fn basic_test_2() {
        let test = String::from("2 3
        7 7 7
        7 7 10");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "TAK
1 3
");
    }

    #[test]
    fn basic_test_3() {
        let test = String::from("3 3
        1 2 3
        1 2 3
        0 0 0
        ");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res,
                  "TAK
2 1 1
");
    }

    
}
