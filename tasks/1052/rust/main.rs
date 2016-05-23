use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
pub struct F32Key(u32);

impl F32Key {
    pub fn new(mut val: f32) -> F32Key {
        if val.is_nan() { val = std::f32::NAN } // make all NaNs have the same representation
        unsafe { F32Key(std::mem::transmute(val)) }
    }
    pub fn get(self) -> f32 {
        unsafe { std::mem::transmute(self) }
    }

    pub fn set(&mut self, mut val : f32) {
        if val.is_nan() { val = std::f32::NAN  } // make all NaNs have the same representation
        unsafe { *self = std::mem::transmute(val) }
    }
}

struct LinesKeeper {
    pub lines: HashMap<(F32Key, F32Key), HashSet<(i32, i32)>>,
}

impl LinesKeeper {
    pub fn new() -> LinesKeeper {
        LinesKeeper { lines: HashMap::new() }
    }
    pub fn add_line(&mut self, a: f32, b: f32, p1: (i32, i32), p2: (i32, i32)) -> usize {
        let k = (F32Key::new(a), F32Key::new(b));

        let mut v = self.lines.entry(k).or_insert(HashSet::new());

        v.insert(p1);
        v.insert(p2);

        v.len()

    }
}

fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut coordinates = Vec::new();

    for _ in 1..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut s = input.trim().split(' ');

        let a_str = s.next().unwrap();
        let x: i32 = a_str.trim().parse().unwrap();

        let b_str = s.next().unwrap();
        let y: i32 = b_str.trim().parse().unwrap();

        coordinates.push((x, y));
    }

    let mut res = 0usize;

    let mut x_axis = HashSet::new();
    let mut y_axis = HashSet::new();

    let mut ln_keeper = LinesKeeper::new();

    for i in 0usize..n - 2 {
        for j in i + 1..n - 1 {

            let p1 = coordinates[i];
            let p2 = coordinates[j];

            let (x1, y1) = p1;
            let (x2, y2) = p2;

            if x1 == x2 {
                if x1 == 0 {
                    x_axis.insert(p1);
                    x_axis.insert(p2);
                    if x_axis.len() > res {
                        res = x_axis.len();
                    }
                } else {
                    let nres = ln_keeper.add_line(1f32 / x1 as f32, 0f32, p1, p2);
                    if nres > res {
                        res = nres;
                    }

                }
            } else if y1 == y2 {
                if y1 == 0 {
                    y_axis.insert(p1);
                    y_axis.insert(p2);
                    if y_axis.len() > res {
                        res = y_axis.len();
                    }
                } else {
                    let nres = ln_keeper.add_line(0f32, 1f32 / y1 as f32, p1, p2);
                    if nres > res {
                        res = nres;
                    }
                }
            } else {
                let la = (y2 - y1) as f32 / (x1 - x2) as f32;
                let b = 1f32 / (la * x1 as f32 + y1 as f32) as f32;
                let nres = ln_keeper.add_line(b * la, b, p1, p2);
                if nres > res {
                    res = nres;
                }
            }
        }
    }

    write!(output, "{}", res).expect("valid output");

    // println!("{}", res);


}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use solve;

    #[test]
    fn basic_test() {
        let mut f = File::open("../input.txt").expect("correct test");
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut f, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "5");
    }
}
