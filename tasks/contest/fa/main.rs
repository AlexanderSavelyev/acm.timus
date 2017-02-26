use std::io::{self, BufReader};
use std::io::prelude::*;


#[derive(Debug)]
struct Gene {
    idx: u32,
    left: u32,
    right: u32,
    len: u32,
}

fn intersects(left:u32, right:u32, gene: &Gene) ->bool {
    if left >= gene.left {
        if left - gene.left <= gene.len {
            return true;
        }
    } else if right >= gene.left {
        if right - gene.left <= gene.len {
            return true;
        }
        if right >= gene.right && left <= gene.left {
            return true;
        }
    } 
    return false;
}

fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    let mut genes: Vec<Gene> = Vec::new();
    let mut max_right: Vec<u32> = Vec::new();
    let mut g_count: Vec<u32> = Vec::new();


    reader.read_line(&mut input).unwrap();
    let genes_len:u32;
    let reads_len:u32;
    {
        let mut s = input.trim().split(' ');

        genes_len = s.next().unwrap().parse().unwrap();
        reads_len = s.next().unwrap().parse().unwrap();
    }
    for i in 0..genes_len {
        input.clear();
        reader.read_line(&mut input).unwrap();

        let mut s = input.trim().split(' ');
        g_count.push(0);

        let mut next_gene = s.next();
        while next_gene.is_some() {
            let left: u32 = next_gene.unwrap().parse().unwrap();
            next_gene = s.next();
            let right: u32 = next_gene.unwrap().parse().unwrap();

            genes.push(Gene{idx: i, left: left, right: right, len: (right-left)});
            next_gene = s.next();
        }
    }

    genes.sort_by(|a, b| a.left.cmp(&b.left));
    let mut max_r = 0u32;
    for g in &genes {
        if g.right > max_r {
            max_r = g.right;
        }
        max_right.push(max_r);
    }
    // //println!("{:?}", genes);

    let mut gene_idx: usize;
    for _ in 0..reads_len {
        input.clear();
        reader.read_line(&mut input).unwrap();

        let mut s = input.trim().split(' ');
        gene_idx = 0;
        let mut next_read = s.next();
        let mut cur_gene: Option<u32> = None;
        let mut has_only_one = true;

        while next_read.is_some() {
            let left: u32 = next_read.unwrap().parse().unwrap();
            next_read = s.next();
            let right: u32 = next_read.unwrap().parse().unwrap();

            //println!("[{}; {}]", left, right);
            while gene_idx < genes.len() {
                let next_gene = &genes[gene_idx];
                if next_gene.left <= right {
                    //println!("go inside");
                    if intersects(left, right, next_gene) {
                        if cur_gene.is_some() && cur_gene.unwrap() != next_gene.idx {
                            has_only_one = false;
                            break;
                        }
                        cur_gene = Some(next_gene.idx);
                    }
                    gene_idx += 1;
                } else {
                    break;
                }
            }
            if gene_idx >= genes.len() && !has_only_one{
                break;
            }
            //println!("{:?}", cur_gene);
            next_read = s.next();
        }

        if has_only_one && cur_gene.is_some() {
            let idx = cur_gene.unwrap() as usize;
            let g = g_count[idx];
            g_count[idx] = g + 1;
        }

    }

    for c in &g_count {
        writeln!(output, "{}", *c).expect("correct output");
    }

}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use solve;

    #[test]
    fn basic_test1() {
        let test = String::from("2 4
1100 1100 2000 2400
3000 3300
1100 1160
1190 1200 2000 2050
3280 3300 3500 3550
1500 1560");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "2\n1\n");
    }
    #[test]
    fn basic_test2() {
        let test = String::from("2 1
1000 2000 3000 4000
1500 3500
2000 3000");
        //let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        assert_eq!(res, "0\n0\n");
    }
}
