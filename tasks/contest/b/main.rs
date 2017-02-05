use std::io::{self, BufReader};
use std::io::prelude::*;

const ADENINE: u8 = 9;
const URACIL: u8 = 6;
const CYTOSINE: u8 = 5;
const GUANINE: u8 = 10;

static mut counter:usize = 0;
static mut level: usize = 0;

struct Solver {
    rna: Vec<u8>,
    a_count: Vec<i32>,
    u_count: Vec<i32>,
    c_count: Vec<i32>,
    g_count: Vec<i32>,
    verbose: bool,
}

impl Solver {
    fn new(n: usize, v: bool) -> Solver {
        let mut rna: Vec<u8> = Vec::with_capacity(n);
        let mut a_count: Vec<i32> = Vec::with_capacity(n + 1);
        let mut u_count: Vec<i32> = Vec::with_capacity(n + 1);
        let mut c_count: Vec<i32> = Vec::with_capacity(n + 1);
        let mut g_count: Vec<i32> = Vec::with_capacity(n + 1);
        rna.resize(n, 0);
        a_count.resize(n + 1, 0);
        u_count.resize(n + 1, 0);
        c_count.resize(n + 1, 0);
        g_count.resize(n + 1, 0);
        Solver {
            rna: rna,
            a_count: a_count,
            u_count: u_count,
            c_count: c_count,
            g_count: g_count,
            verbose: v,
        }
    }

    fn is_perfect(&self, from:usize, to: usize, almost: bool) -> bool {
        if self.verbose {
            unsafe {print!("{:<1$}", "", level);}
            println!("is_perfect from = {:?} to = {:?} almost = {:?}", from, to, almost);
        }
        unsafe {
            counter += 1;
            level += 1;
            if counter > 10000 {
                // return false;
                panic!("too much");
            }
        }
        // if to >= self.rna.len() {
        //     return true;
        // }
        if from > to {
            panic!("from > to");
        }
        let cnt: i32 = to as i32 - from as i32 + 1;
        if cnt <= 0 {
            panic!("from > to");
        }

        if cnt % 2 == 0 && almost {
            panic!("almost for even");
        }
        if cnt % 2 != 0 && !almost {
            panic!("not almost for odd");
        }
        if cnt == 1 {
            unsafe {
                level -=1;
            }
            return almost;
        }
        if cnt == 2 {
            unsafe {
                level -=1;
            }
            return self.rna[from] & self.rna[to] == 0
        }
        let mut from_idx = from;
        let mut right_len;
        let mut left_len;
        let mut left_almost;
        let mut right_almost;
        for to_idx in (from_idx + 1 .. to + 1).rev() {
            if self.rna[from_idx] & self.rna[to_idx] == 0 {
                left_len = to_idx - from_idx + 1;
                right_len = to - to_idx;
                if almost {
                    if left_len % 2 == 0 {
                        left_almost = false;
                        right_almost = true;
                    } else {
                        right_almost = false;
                        left_almost = true;
                    }
                } else {
                    if (right_len % 2 != 0) || (left_len %2 != 0) {
                        continue;
                    }
                    left_almost = false;
                    right_almost = false;
                }
                if left_len > 2 && !self.is_possible(from_idx + 1, to_idx - 1, left_almost) {
                    continue;
                }
                if to_idx + 1 <= to && !self.is_possible (to_idx + 1, to, right_almost) {
                    continue;
                }
                if left_len > 2 && !self.is_perfect(from_idx + 1, to_idx - 1, left_almost) {
                    continue;
                }
                if to_idx + 1 <= to && !self.is_perfect (to_idx + 1, to, right_almost) {
                    continue;
                }
                unsafe {
                    level -=1;
                }
                return true;
            }
        }
        if almost {
            from_idx = from + 1;
            // if self.verbose {
            //     if cnt == 3 {
            //         println!("cnt = 3 {} {}", from_idx, to);
            //     }
            // }
            for to_idx in (from_idx + 1 .. to + 1).rev() {
                if self.rna[from_idx] & self.rna[to_idx] == 0 {
                    left_len = to_idx - from_idx + 1;
                    right_len = to - to_idx;
                    if (right_len % 2 !=0) || (left_len % 2 != 0) {
                        continue;
                    }
                    if left_len > 2 && !self.is_possible(from_idx + 1, to_idx - 1, false) {
                        continue;
                    }
                    if to_idx + 1 <= to && !self.is_possible (to_idx + 1, to, false) {
                        continue;
                    }
                    if left_len > 2 && !self.is_perfect(from_idx + 1, to_idx - 1, false) {
                        continue;
                    }
                    if to_idx + 1 <= to && !self.is_perfect (to_idx + 1, to, false) {
                        continue;
                    }
                    unsafe {
                        level -=1;
                    }
                    return true;
                }
            }
        }
        unsafe {
            level -=1;
        }
        return false;
    }

    fn is_possible(&self, from: usize, to: usize, almost: bool)->bool {
        // if self.verbose {
        //     println!("is_possible from = {:?} to = {:?} almost = {:?}", from, to, almost);
        // }
        // if to >= self.rna.len() {
        //     return true;
        // }
        let cnt: i32 = to as i32 - from as i32 + 1;
        if cnt <= 0 {
            return true;
        }
        if cnt == 1 {
            return almost;
        }
        let a_count = self.a_count[to + 1] - self.a_count[from];
        let u_count = self.u_count[to + 1] - self.u_count[from];
        let c_count = self.c_count[to + 1] - self.c_count[from];
        let g_count = self.g_count[to + 1] - self.g_count[from];
        let au_diff = a_count - u_count;
        let cg_diff = c_count - g_count;
        let sum = au_diff.abs() + cg_diff.abs();
        if almost {
            return sum == 1;
        } else {
            return sum == 0;
        }
    }
}


fn solve1(input: &mut Read, output: &mut Write, verbose: bool) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let lbytes = input.trim().as_bytes();
    let rna_len = lbytes.len();
    let mut solver = Solver::new(rna_len, verbose);

    for i in 0..lbytes.len() {
        solver.a_count[i + 1] = solver.a_count[i];
        solver.u_count[i + 1] = solver.u_count[i];
        solver.c_count[i + 1] = solver.c_count[i];
        solver.g_count[i + 1] = solver.g_count[i];
        match lbytes[i] {
            b'A' => {
                solver.rna[i] = ADENINE;
                solver.a_count[i + 1] += 1
            }
            b'U' => {
                solver.rna[i] = URACIL;
                solver.u_count[i + 1] += 1
            }
            b'C' => {
                solver.rna[i] = CYTOSINE;
                solver.c_count[i + 1] += 1
            }
            b'G' => {
                solver.rna[i] = GUANINE;
                solver.g_count[i + 1] += 1
            }
            _ => panic!("unknown {}", lbytes[i]),
        }
    }

    let almost = rna_len % 2 ==1;

    if !solver.is_possible(0, rna_len -1 , almost) {
        write!(output, "imperfect").expect("correct output");
        return; 
    }

    let is_perfect = solver.is_perfect(0, rna_len -1, almost);

    if is_perfect {
        if almost {
            write!(output, "almost perfect").expect("correct output");
        } else {
            write!(output, "perfect").expect("correct output");
        }
    } else {
        write!(output, "imperfect").expect("correct output");
    }

}

fn solve(input: &mut Read, output: &mut Write, verbose: bool) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let lbytes = input.trim().as_bytes();
    let rna_len = lbytes.len();
    let mut rna: Vec<u8> = Vec::with_capacity(rna_len);
    rna.resize(rna_len, 0);

    for i in 0..lbytes.len() {
        match lbytes[i] {
            b'A' => {
                rna[i] = ADENINE;
            }
            b'U' => {
                rna[i] = URACIL;
            }
            b'C' => {
                rna[i] = CYTOSINE;
            }
            b'G' => {
                rna[i] = GUANINE;
            }
            _ => panic!("unknown {}", lbytes[i]),
        }
    }

    let mut rna_stack:Vec<u8> = Vec::with_capacity(rna_len);
    rna_stack.resize(rna_len, 0);
    let mut stack_len = 0usize;
    if rna_len % 2 == 0 {
        for i in 0 .. rna.len() {
            if stack_len == 0 {
                rna_stack[stack_len] = rna[i];
                stack_len += 1;
                continue;
            }

            if rna[i] & rna_stack[stack_len - 1] == 0 {
                stack_len -= 1;
            } else {
                rna_stack[stack_len] = rna[i];
                stack_len += 1;
            }
        }

        if stack_len == 0 {
            write!(output, "perfect").expect("correct output");
        } else {
            write!(output, "imperfect").expect("correct output");
        }
    } else {
        for i in 0 .. rna.len() {
            if stack_len == 0 {
                rna_stack[stack_len] = rna[i];
                stack_len += 1;
                continue;
            }

            if rna[i] & rna_stack[stack_len - 1] == 0 {
                stack_len -= 1;
            } else {
                rna_stack[stack_len] = rna[i];
                stack_len += 1;
            }
        }

        if stack_len % 2 == 0 {
            write!(output, "imperfect").expect("correct output");
        } else {
            if stack_len == 1 {
                write!(output, "almost perfect").expect("correct output");
                return;
            }
            let start = stack_len / 2 + 1;
            let mut last = stack_len / 2 - 1;
            for i in start .. stack_len {
                if rna_stack[i] & rna_stack[last] == 0 {
                    if last > 0 {
                        last -= 1;
                    }
                } else {
                    write!(output, "imperfect").expect("correct output");
                    return;
                }
            }
            write!(output, "almost perfect").expect("correct output");
        }
        
        
    }

    // if !solver.is_possible(0, rna_len -1 , almost) {
    //     write!(output, "imperfect").expect("correct output");
    //     return; 
    // }

    // let is_perfect = solver.is_perfect(0, rna_len -1, almost);

    // if is_perfect {
    //     if almost {
    //         write!(output, "almost perfect").expect("correct output");
    //     } else {
    //         write!(output, "perfect").expect("correct output");
    //     }
    // } else {
    //     write!(output, "imperfect").expect("correct output");
    // }

}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout(), true);
}

#[cfg(test)]
mod tests {
    // use std::fs::File;
    use solve;
    use ADENINE;
    use URACIL;
    use CYTOSINE;
    use GUANINE;
    #[test]
    fn test_pairs() {
        assert_eq!(0, ADENINE & URACIL);
        assert_eq!(0, CYTOSINE & GUANINE);
        assert!(ADENINE & CYTOSINE > 0);
        assert!(ADENINE & GUANINE > 0);
        assert!(URACIL & CYTOSINE > 0);
        assert!(URACIL & GUANINE > 0);
    }

    #[test]
    fn basic_test1() {
        let test = String::from("UGCA");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf, false);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "perfect");
    }
    #[test]
    fn basic_test2() {
        let test = String::from("AGUCU");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf, false);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "almost perfect");
    }
    #[test]
    fn basic_test3() {
        let test = String::from("CAGUU");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf, true);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "imperfect");
    }
    #[test]
    fn basic_test25() {
        let test = String::from("UAUUAACUAGCGCGCAUAUAAUUAUGGCCGGCCGCGCACUAUAGUCGACUCUGACGAUCUGACAUGCGUAUCCGGCACGCGGCGUAAGCAUUGCUAGAUCG");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf, true);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "imperfect");
    }
}
