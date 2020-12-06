use std::cmp;

#[allow(dead_code)]
const ADDRESS_BITS_PER_WORD: u16 = 6;
#[allow(dead_code)]
const BITS_PER_WORD: u16 = 1 << ADDRESS_BITS_PER_WORD;
#[allow(dead_code)]
const WORD_MASK: u64 = 0xFFFFFFFFFFFFFFFF;
#[allow(dead_code)]
struct DBitset {
    words_in_use: usize,
    words: Vec<u64>,
}
#[allow(dead_code,unused_parens)]
impl DBitset {
    fn word_index(nbits: usize) -> usize {
        nbits >> ADDRESS_BITS_PER_WORD
    }
    fn new(nbits: usize) -> DBitset {
        let l = DBitset::word_index(nbits - 1) + 1;
        let mut w = Vec::with_capacity(l);
        w.resize(l, 0);
        DBitset {
            words_in_use: 0,
            words: w,
        }
    }
    fn is_empty(&self) -> bool {
        self.words_in_use == 0
    }
    fn set(&mut self, bit_idx: usize) {
        let wordindex = DBitset::word_index(bit_idx);
        let mut bit = bit_idx;
        bit -= (wordindex << ADDRESS_BITS_PER_WORD);
        self.expand_to(wordindex);
        self.words[wordindex] |= (1u64 << bit);
    }
    fn setc(&mut self, bit_idx: usize) -> bool {
        let wordindex = DBitset::word_index(bit_idx);
        let mut bit = bit_idx;
        bit -= (wordindex << ADDRESS_BITS_PER_WORD);
        self.expand_to(wordindex);
        let w = self.words[wordindex];
        self.words[wordindex] |= (1u64 << bit);
        return w != self.words[wordindex];
    }

    fn reset(&mut self, bit_idx: usize) {
        let wordindex = DBitset::word_index(bit_idx);
        if wordindex >= self.words_in_use {
            return;
        }
        let mut bit = bit_idx;
        bit -= (wordindex << ADDRESS_BITS_PER_WORD);

        self.words[wordindex] &= !(1u64 << bit);
        self.recalculate_words_in_use();
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
        self.words_in_use = 0;
        for i in (0..self.words.len()).rev() {
            if self.words[i] != 0 {
                self.words_in_use = i + 1;
                break;
            }
        }
    }

    fn and_with(&mut self, set: &DBitset) {
        let mut word_len = self.words_in_use;
        if self.words_in_use > set.words_in_use {
            word_len = set.words_in_use;
            for i in word_len..self.words_in_use {
                self.words[i] = 0;
            }
        }

        for i in 0..word_len {
            self.words[i] &= set.words[i];
        }
        self.recalculate_words_in_use();
    }
    fn and_not_with(&mut self, set: &DBitset) {
        let w_min = cmp::min(self.words_in_use, set.words_in_use);
        for i in 0..w_min {
            self.words[i] &= !set.words[i];
        }
        self.recalculate_words_in_use();
    }
    fn is_subset_of(&self, set: &DBitset) -> bool {
        if self.words_in_use > set.words_in_use {
            return false;
        }
        for i in 0..self.words_in_use {
            if (self.words[i] & (!set.words[i])) != 0 {
                return false;
            }
        }
        return true;
    }
    fn or_with(&mut self, set: &DBitset) -> bool {
        let mut changed = false;
        if self.words_in_use < set.words_in_use {
            self.words_in_use = set.words_in_use;
        }
        if self.words.len() < self.words_in_use {
            self.words.resize(self.words_in_use, 0);
        }
        let w_min = cmp::min(self.words_in_use, set.words_in_use);
        for i in 0..w_min {
            let w = self.words[i];
            self.words[i] |= set.words[i];
            if w != self.words[i] {
                changed = true;
            }
        }
        return changed;
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

    // let mut bit = cell.next_set_bit(0);
    // while bit.is_some() {
    //     let b = bit.unwrap();
    //     bit = cell.next_set_bit(b + 1);
    // }
    //
}


impl<'a> IntoIterator for &'a DBitset {
    type Item = usize;
    type IntoIter = BitsetIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BitsetIterator {
            bitset: self,
            index: None,
        }
    }
}

struct BitsetIterator<'a> {
    bitset: &'a DBitset,
    index: Option<usize>,
}

impl<'a> Iterator for BitsetIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let start_idx = match self.index {
            Some(idx) => {
                idx + 1
            }, 
            None => {
                0
            }
        };
        match self.bitset.next_set_bit(start_idx) {
            Some(bit) => {
                self.index.replace(bit);
                return Some(bit);
            },
            None => {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // rustc --test d_bitset.rs; ./d_bitset --nocapture
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
    fn test_bitset5() {
        let mut b = DBitset::new(1024);
        b.set(0);
        b.set(1);
        b.set(5);
        b.set(6);
        b.set(200);
        b.set(10000);
        b.set(50000);
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
        assert_eq!(Some(10000), bit);
        bit = b.next_set_bit(bit.unwrap() + 1);
        assert_eq!(Some(50000), bit);
        bit = b.next_set_bit(bit.unwrap() + 1);
        assert_eq!(None, bit);
        let mut b2 = DBitset::new(1024);
        b2.or_with(&b);

        bit = b2.next_set_bit(0);
        assert_eq!(Some(0), bit);
        bit = b2.next_set_bit(bit.unwrap() + 1);
        assert_eq!(Some(1), bit);
        bit = b2.next_set_bit(bit.unwrap() + 1);
        assert_eq!(Some(5), bit);
        bit = b2.next_set_bit(bit.unwrap() + 1);
        assert_eq!(Some(6), bit);
        bit = b2.next_set_bit(bit.unwrap() + 1);
        assert_eq!(Some(200), bit);
        bit = b2.next_set_bit(bit.unwrap() + 1);
        assert_eq!(Some(10000), bit);
        bit = b2.next_set_bit(bit.unwrap() + 1);
        assert_eq!(Some(50000), bit);
        bit = b2.next_set_bit(bit.unwrap() + 1);
        assert_eq!(None, bit);
        // println!("{:?}", b.next_set_bit(0));
    }

    #[test]
    fn test_bitset6() {
        let mut b = DBitset::new(1000);
        b.set(0);
        b.set(1);
        b.set(5);
        b.set(6);
        b.set(200);

        let mut res: Vec<String> = Vec::new();
        for bit in &b {
            res.push(bit.to_string());
        }
        assert_eq!("0 1 5 6 200", res.join(" "));
    }
}
