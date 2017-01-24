use std::io::{self, BufReader};
use std::io::prelude::*;
#[allow(dead_code)]
const ADDRESS_BITS_PER_WORD: u16 = 6;
#[allow(dead_code)]
const BITS_PER_WORD: u16 = 1 << ADDRESS_BITS_PER_WORD;
#[allow(dead_code)]
const WORD_MASK: u64 = 0xFFFFFFFFFFFFFFFF;
#[allow(dead_code)]
struct DBitset {
    words_in_use: usize,
    length: usize,
    words: Vec<u64>,
}
#[allow(dead_code)]
impl DBitset {
    fn word_index(nbits: usize) -> usize {
        nbits >> ADDRESS_BITS_PER_WORD
    }
    fn new(nbits: usize) -> DBitset {
        let l = DBitset::word_index(nbits - 1) + 1;
        let mut w = Vec::with_capacity(l);
        w.resize(l, 0);
        // for _ in 0..l {
        //     w.push(0);
        // }
        DBitset {
            words_in_use: 0,
            length: l,
            words: w,
        }
    }
    fn set(&mut self, bit_idx: usize) {
        let wordindex = DBitset::word_index(bit_idx);
        let mut bit = bit_idx;
        bit -= (wordindex << ADDRESS_BITS_PER_WORD);
        self.expand_to(wordindex);
        self.words[wordindex] |= (1u64 << bit);
    }

    fn get(&self, bit_idx: usize) -> bool {
        let word_index = DBitset::word_index(bit_idx);
        let mut bit = bit_idx;
        bit -= word_index << ADDRESS_BITS_PER_WORD;
        (word_index < self.words_in_use) && ((self.words[word_index] & (1u64 << bit)) != 0)
    }
    fn expand_to(&mut self, word_idx: usize) {
        let words_required = word_idx + 1;
        if self.words_in_use < words_required {
            self.words_in_use = words_required;
        }
        if self.words.len() < words_required {
            self.words.resize(words_required, 0);
        }
    }

    fn recalculate_words_in_use(&mut self) {
        for i in (0..self.length).rev() {
            if self.words[i] != 0 {
                self.words_in_use = i + 1;
                break;
            }
        }

    }
    fn is_subset_of(&self, set: &DBitset) -> bool {
        for i in 0..self.words_in_use {
            if (self.words[i] & (!set.words[i])) != 0 {
                return false;
            }
        }
        return true;
    }
    fn or_with(&mut self, set: &DBitset) {
        if self.words_in_use < set.words_in_use {
            self.words_in_use = set.words_in_use;
        }
        if self.words.len() < self.words_in_use {
            self.words.resize(self.words_in_use, 0);
        }
        for i in 0..self.words_in_use {
            self.words[i] |= set.words[i];
        }
    }

    fn least_significant_bit_position(m: u64) -> Option<usize> {
        let mut n = m;
        if n == 0 {
            return None;
        }

        let mut pos = 63usize;
        if n & 0x00000000FFFFFFFFu64 != 0 {
            pos -= 32;
        } else {
            n >>= 32;
        }
        if n & 0x000000000000FFFFu64 != 0 {
            pos -= 16;
        } else {
            n >>= 16;
        }
        if n & 0x00000000000000FFu64 != 0 {
            pos -= 8;
        } else {
            n >>= 8;
        }
        if n & 0x000000000000000Fu64 != 0 {
            pos -= 4;
        } else {
            n >>= 4;
        }
        if n & 0x0000000000000003u64 != 0 {
            pos -= 2;
        } else {
            n >>= 2;
        }
        if n & 0x0000000000000001u64 != 0 {
            pos -= 1;
        }
        return Some(pos);
    }

    fn next_set_bit(&self, from_index: usize) -> Option<usize> {
        let mut from_idx = from_index;
        let mut u = DBitset::word_index(from_idx);
        if u >= self.words_in_use {
            return None;
        }
        from_idx -= (u << ADDRESS_BITS_PER_WORD);

        let mut word = self.words[u] & (WORD_MASK << from_idx);
        while word == 0 {
            u += 1;
            if u >= self.words_in_use {
                return None;
            }
            word = self.words[u];
        }
        let bit = u << ADDRESS_BITS_PER_WORD;
        let lbit = DBitset::least_significant_bit_position(word);

        if bit == 0 && lbit.is_none() {
            return None;
        }

        return Some(bit + lbit.unwrap());
    }
}

struct Reaction {
    left: DBitset,
    right: DBitset,
}

impl Reaction {
    fn new() -> Reaction {
        Reaction {
            left: DBitset::new(1024),
            right: DBitset::new(1024),
        }
    }
}

fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    // let mut chemicals: Vec<usize> = Vec::new();
    let mut cell = DBitset::new(1024);
    let mut reactions: Vec<Reaction> = Vec::new();

    for nc in input.trim().split(' ') {
        let n: usize = nc.trim().parse().unwrap();
        cell.set(n);
        // chemicals.push(n);
    }
    // println!("{:?}", chemicals);
    // let n: i32 = input.trim().parse().unwrap();
    for reaction in reader.lines().map(|r| r.unwrap()) {
        println!("{:?}", reaction);
        let mut r = Reaction::new();
        let mut parts = reaction.trim().split("->");

        let left_str = parts.next().unwrap();
        for lc in left_str.trim().split('+').map(|r| r.trim()) {
            let ln: usize = lc.parse().unwrap();
            r.left.set(ln);
        }
        let right_str = parts.next().unwrap();
        for lc in right_str.trim().split('+').map(|r| r.trim()) {
            let ln: usize = lc.parse().unwrap();
            r.right.set(ln);
        }
        reactions.push(r);
        // println!("{:?}", right_str);
        // let a: i32 = a_str.trim().parse().unwrap();
    }


    // for _ in 0..n {
    //     input.clear();
    //     reader.read_line(&mut input).unwrap();
    //     println!("{:?}", input);
    // let mut s = input.trim().split(' ');

    // let a_str = s.next().unwrap();
    // let a: i32 = a_str.trim().parse().unwrap();

    // let b_str = s.next().unwrap();
    // let b: i32 = b_str.trim().parse().unwrap();

    // println!("{} {}", a,b);
    // }

    // println!("{}", n);


}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use solve;
    use DBitset;

    #[test]
    fn test_bitset1() {
        let mut b = DBitset::new(1000);
        assert_eq!(false, b.get(65));
        b.set(65);
        assert_eq!(true, b.get(65));
    }

    #[test]
    fn test_bitset2() {
        let mut b = DBitset::new(1000);
        b.set(1025);
        assert_eq!(true, b.get(1025));
    }
    #[test]
    fn test_bitset3() {
        let mut b1 = DBitset::new(1000);
        let mut b2 = DBitset::new(1000);
        b1.set(1);
        b1.set(5);
        b1.set(6);
        b1.set(200);
        b2.set(1);
        b2.set(5);
        b2.set(6);
        b2.set(200);
        b2.set(260);
        b2.set(10);
        assert_eq!(true, b1.is_subset_of(&b2));
        b1.set(7);
        assert_eq!(false, b1.is_subset_of(&b2));
    }

    #[test]
    fn test_bitset4() {
        let mut b = DBitset::new(1000);
        b.set(0);
        b.set(1);
        b.set(5);
        b.set(6);
        b.set(200);
        let mut bit = b.next_set_bit(0);
        assert_eq!(Some(0), bit);
        bit = b.next_set_bit(bit.unwrap() + 1);
        assert_eq!(Some(1), bit);
        bit = b.next_set_bit(bit.unwrap() + 1);
        assert_eq!(Some(5), bit);
        bit = b.next_set_bit(bit.unwrap() + 1);
        assert_eq!(Some(6), bit);
        bit = b.next_set_bit(bit.unwrap() + 1);
        assert_eq!(Some(200), bit);
        bit = b.next_set_bit(bit.unwrap() + 1);
        assert_eq!(None, bit);

        // println!("{:?}", b.next_set_bit(0));
    }

    #[test]
    fn basic_test1() {
        let test = String::from("4
4+6->1
2->3+5
4->6
6+4->5");
        // let mut f = File::open("../input.txt").expect("correct test");
        let testb = test.into_bytes();
        let mut test_r = testb.as_slice();
        let mut buf: Vec<u8> = Vec::new();
        solve(&mut test_r, &mut buf);

        let res = String::from_utf8(buf).expect("valid string");
        // assert_eq!(res,
        //                   "2297.0716
        // 936297014.1164
        // 0.0000
        // 37.7757
        // ");
    }
}
