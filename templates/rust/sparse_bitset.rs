
use std::cmp;

#[allow(dead_code)]
const ADDRESS_BITS_PER_WORD: u16 = 6;
#[allow(dead_code)]
const BITS_PER_WORD: u16 = 1 << ADDRESS_BITS_PER_WORD;
#[allow(dead_code)]
const WORD_MASK: u64 = 0xFFFFFFFFFFFFFFFF;

#[allow(dead_code)]
#[derive(Debug)]
struct SparseMap {
    num_bits: u8,
    position: u32,
    reference: u32,
}

#[allow(dead_code)]
struct SparseBitset {
    words_map: Vec<SparseMap>,
    words: Vec<u64>,
    num_bits: usize,
}
#[allow(dead_code)]
impl SparseMap {
    fn new(num_bits: u8, position: u32, reference: u32) -> SparseMap {
        SparseMap {
            num_bits: num_bits,
            position: position,
            reference: reference,
        }
    }
}
#[allow(dead_code,unused_parens)]
impl SparseBitset {
    
    fn new(nbits: usize) -> SparseBitset {
        let last_word = SparseBitset::word_index(nbits - 1);
        let mut words_map = Vec::new();
        words_map.push(SparseMap::new(0, 0, last_word as u32));
        SparseBitset {
            words_map: words_map,
            words: Vec::new(),
            num_bits: 0,
        }
    }

    fn word_index(nbits: usize) -> usize {
        nbits >> ADDRESS_BITS_PER_WORD
    }

    // fn is_empty(&self) -> bool {
    //     self.words_in_use == 0
    // }
    fn word_map_contains(&self, mid: usize, word_idx: u32) -> bool {
        if self.words_map[mid].num_bits > 0 {
            return self.words_map[mid].position == word_idx;
        } else {
            return self.words_map[mid].position <= word_idx && word_idx <= self.words_map[mid].reference;
        }
    }
    fn find_map_idx(&self, word_idx: u32) -> usize {
        let mut size = self.words_map.len();
        let mut base = 0_usize;
        println!("find word idx {}", word_idx);

        while size >= 1 {
            // mid: [base..size)
            let half = size / 2;
            let mid = base + half;
            if self.word_map_contains(mid, word_idx) {
                return mid;
            } 
            if self.words_map[mid].position < word_idx  {
                base = mid
            }
            size -= half;
        }
        panic!("incorrect state");
    }
    fn split_words(&mut self, map_idx: usize, word_idx: u32) -> usize {
        let old_position = self.words_map[map_idx].position;
        let old_reference = self.words_map[map_idx].reference;
        let ref_idx = self.words.len() as u32;
        self.words.push(0);
        if old_position == word_idx && old_reference == word_idx {
            self.words_map[map_idx].reference = ref_idx;
        } else {
            let new_map = SparseMap::new(0, word_idx, ref_idx);

            if old_position == word_idx {
                self.words_map[map_idx].position += 1;
                self.words_map.insert(map_idx, new_map);
            } else if old_reference == word_idx {
                self.words_map[map_idx].reference -= 1;
                self.words_map.insert(map_idx + 1, new_map);
                return map_idx + 1;
            } else {
                self.words_map.insert(map_idx, SparseMap::new(0, old_position, word_idx - 1));
                self.words_map[map_idx + 1].position = word_idx + 1;
                self.words_map.insert(map_idx + 1, new_map);
                return map_idx + 1;
            }
            
        }
        return map_idx;
    }

    fn merge_words(&mut self, map_idx: usize, word_idx: u32) {

    }
    fn set(&mut self, bit_idx: usize) -> bool {
        println!("set {}", bit_idx);
        let word_idx = SparseBitset::word_index(bit_idx);
        let mut map_idx = self.find_map_idx(word_idx as u32);
        if self.words_map[map_idx].num_bits == 0 {
            map_idx = self.split_words(map_idx, word_idx as u32);
        }
        let mut bit = bit_idx;
        bit -= (word_idx << ADDRESS_BITS_PER_WORD);
        let word = self.words_map[map_idx].reference as usize;
        let w = self.words[word];
        self.words[word] |= (1u64 << bit);
        if w != self.words[word] {
            self.words_map[map_idx].num_bits += 1;
            self.num_bits += 1;
            return true;
        }
        return false;
    }

    fn reset(&mut self, bit_idx: usize) -> bool {
        println!("reset {}", bit_idx);
        let word_idx = SparseBitset::word_index(bit_idx);
        let mut map_idx = self.find_map_idx(word_idx as u32);

        if self.words_map[map_idx].num_bits == 0 {
            return false;
        }

        let mut bit = bit_idx;
        bit -= (word_idx << ADDRESS_BITS_PER_WORD);
        let word = self.words_map[map_idx].reference as usize;
        let w = self.words[word];
        self.words[word] &= !(1u64 << bit);
        if w != self.words[word] {
            self.words_map[map_idx].num_bits -= 1;
            self.num_bits -= 1;
            if self.words_map[map_idx].num_bits == 0 {
                self.merge_words(map_idx, word_idx as u32);
            }
            return true;
        }

        return false;
    }

    // fn get(&self, bit_idx: usize) -> bool {
    //     let word_index = DBitset::word_index(bit_idx);
    //     let mut bit = bit_idx;
    //     bit -= word_index << ADDRESS_BITS_PER_WORD;
    //     (word_index < self.words_in_use) && ((self.words[word_index] & (1u64 << bit)) != 0)
    // }
    // fn expand_to(&mut self, word_idx: usize) {
    //     let words_required = word_idx + 1;
    //     if self.words_in_use < words_required {
    //         self.words_in_use = words_required;
    //     }
    //     if self.words.len() < words_required {
    //         self.words.resize(words_required, 0);
    //     }
    // }

    // fn recalculate_words_in_use(&mut self) {
    //     self.words_in_use = 0;
    //     for i in (0..self.words.len()).rev() {
    //         if self.words[i] != 0 {
    //             self.words_in_use = i + 1;
    //             break;
    //         }
    //     }
    // }

    // fn and_with(&mut self, set: &DBitset) {
    //     let mut word_len = self.words_in_use;
    //     if self.words_in_use > set.words_in_use {
    //         word_len = set.words_in_use;
    //         for i in word_len..self.words_in_use {
    //             self.words[i] = 0;
    //         }
    //     }

    //     for i in 0..word_len {
    //         self.words[i] &= set.words[i];
    //     }
    //     self.recalculate_words_in_use();
    // }
    // fn and_not_with(&mut self, set: &DBitset) {
    //     let w_min = cmp::min(self.words_in_use, set.words_in_use);
    //     for i in 0..w_min {
    //         self.words[i] &= !set.words[i];
    //     }
    //     self.recalculate_words_in_use();
    // }
    // fn is_subset_of(&self, set: &DBitset) -> bool {
    //     if self.words_in_use > set.words_in_use {
    //         return false;
    //     }
    //     for i in 0..self.words_in_use {
    //         if (self.words[i] & (!set.words[i])) != 0 {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    // fn or_with(&mut self, set: &DBitset) -> bool {
    //     let mut changed = false;
    //     if self.words_in_use < set.words_in_use {
    //         self.words_in_use = set.words_in_use;
    //     }
    //     if self.words.len() < self.words_in_use {
    //         self.words.resize(self.words_in_use, 0);
    //     }
    //     let w_min = cmp::min(self.words_in_use, set.words_in_use);
    //     for i in 0..w_min {
    //         let w = self.words[i];
    //         self.words[i] |= set.words[i];
    //         if w != self.words[i] {
    //             changed = true;
    //         }
    //     }
    //     return changed;
    // }

    // fn least_significant_bit_position(m: u64) -> Option<usize> {
    //     let mut n = m;
    //     if n == 0 {
    //         return None;
    //     }

    //     let mut pos = 63usize;
    //     if n & 0x00000000FFFFFFFFu64 != 0 {
    //         pos -= 32;
    //     } else {
    //         n >>= 32;
    //     }
    //     if n & 0x000000000000FFFFu64 != 0 {
    //         pos -= 16;
    //     } else {
    //         n >>= 16;
    //     }
    //     if n & 0x00000000000000FFu64 != 0 {
    //         pos -= 8;
    //     } else {
    //         n >>= 8;
    //     }
    //     if n & 0x000000000000000Fu64 != 0 {
    //         pos -= 4;
    //     } else {
    //         n >>= 4;
    //     }
    //     if n & 0x0000000000000003u64 != 0 {
    //         pos -= 2;
    //     } else {
    //         n >>= 2;
    //     }
    //     if n & 0x0000000000000001u64 != 0 {
    //         pos -= 1;
    //     }
    //     return Some(pos);
    // }

    // fn next_set_bit(&self, from_index: usize) -> Option<usize> {
    //     let mut from_idx = from_index;
    //     let mut u = DBitset::word_index(from_idx);
    //     if u >= self.words_in_use {
    //         return None;
    //     }
    //     from_idx -= (u << ADDRESS_BITS_PER_WORD);
    //     let mut word = self.words[u] & (WORD_MASK << from_idx);
    //     while word == 0 {
    //         u += 1;
    //         if u >= self.words_in_use {
    //             return None;
    //         }
    //         word = self.words[u];
    //     }
    //     let bit = u << ADDRESS_BITS_PER_WORD;
    //     let lbit = DBitset::least_significant_bit_position(word);

    //     if bit == 0 && lbit.is_none() {
    //         return None;
    //     }

    //     return Some(bit + lbit.unwrap());
    // }

    // let mut bit = cell.next_set_bit(0);
    // while bit.is_some() {
    //     let b = bit.unwrap();
    //     bit = cell.next_set_bit(b + 1);
    // }
    //
}




// impl<'a> IntoIterator for &'a SparseBitset {
//     type Item = usize;
//     type IntoIter = BitsetIterator<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         BitsetIterator {
//             bitset: self,
//             index: None,
//         }
//     }
// }

// struct SBitsetIterator<'a> {
//     bitset: &'a SparseBitset,
//     index: Option<usize>,
// }

// impl<'a> Iterator for SBitsetIterator<'a> {
//     type Item = usize;
//     fn next(&mut self) -> Option<usize> {
//         let start_idx = match self.index {
//             Some(idx) => {
//                 idx + 1
//             }, 
//             None => {
//                 0
//             }
//         };
//         match self.bitset.next_set_bit(start_idx) {
//             Some(bit) => {
//                 self.index.replace(bit);
//                 return Some(bit);
//             },
//             None => {
//                 return None;
//             }
//         }
//     }


#[cfg(test)]
mod tests {
    // rustc --test sparse_bitset.rs; ./d_bitset --nocapture
    use SparseBitset;

    #[test]
    fn test_bitset1() {
        let mut b = SparseBitset::new(1000);
        // assert_eq!(false, b.get(65));
        b.set(65);
        println!("words_map {:?}", b.words_map);
        println!("words {:?}", b.words);
        println!("num_bits {:?}", b.num_bits);
        b.set(64);
        println!("words_map {:?}", b.words_map);
        println!("words {:?}", b.words);
        println!("num_bits {:?}", b.num_bits);
        b.set(15);
        println!("words_map {:?}", b.words_map);
        println!("words {:?}", b.words);
        println!("num_bits {:?}", b.num_bits);
        b.set(70);
        println!("words_map {:?}", b.words_map);
        println!("words {:?}", b.words);
        println!("num_bits {:?}", b.num_bits);
        b.set(200);
        println!("words_map {:?}", b.words_map);
        println!("words {:?}", b.words);
        println!("num_bits {:?}", b.num_bits);
        // assert_eq!(true, b.get(65));
    }

    // #[test]
    // fn test_bitset2() {
    //     let mut b = DBitset::new(1000);
    //     b.set(1025);
    //     assert_eq!(true, b.get(1025));
    // }
    // #[test]
    // fn test_bitset3() {
    //     let mut b1 = DBitset::new(1000);
    //     let mut b2 = DBitset::new(1000);
    //     b1.set(1);
    //     b1.set(5);
    //     b1.set(6);
    //     b1.set(200);
    //     b2.set(1);
    //     b2.set(5);
    //     b2.set(6);
    //     b2.set(200);
    //     b2.set(260);
    //     b2.set(10);
    //     assert_eq!(true, b1.is_subset_of(&b2));
    //     b1.set(7);
    //     assert_eq!(false, b1.is_subset_of(&b2));
    // }

    // #[test]
    // fn test_bitset4() {
    //     let mut b = DBitset::new(1000);
    //     b.set(0);
    //     b.set(1);
    //     b.set(5);
    //     b.set(6);
    //     b.set(200);
    //     let mut bit = b.next_set_bit(0);
    //     assert_eq!(Some(0), bit);
    //     bit = b.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(1), bit);
    //     bit = b.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(5), bit);
    //     bit = b.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(6), bit);
    //     bit = b.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(200), bit);
    //     bit = b.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(None, bit);

    //     // println!("{:?}", b.next_set_bit(0));
    // }
    // #[test]
    // fn test_bitset5() {
    //     let mut b = DBitset::new(1024);
    //     b.set(0);
    //     b.set(1);
    //     b.set(5);
    //     b.set(6);
    //     b.set(200);
    //     b.set(10000);
    //     b.set(50000);
    //     let mut bit = b.next_set_bit(0);
    //     assert_eq!(Some(0), bit);
    //     bit = b.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(1), bit);
    //     bit = b.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(5), bit);
    //     bit = b.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(6), bit);
    //     bit = b.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(200), bit);
    //     bit = b.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(10000), bit);
    //     bit = b.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(50000), bit);
    //     bit = b.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(None, bit);
    //     let mut b2 = DBitset::new(1024);
    //     b2.or_with(&b);

    //     bit = b2.next_set_bit(0);
    //     assert_eq!(Some(0), bit);
    //     bit = b2.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(1), bit);
    //     bit = b2.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(5), bit);
    //     bit = b2.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(6), bit);
    //     bit = b2.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(200), bit);
    //     bit = b2.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(10000), bit);
    //     bit = b2.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(Some(50000), bit);
    //     bit = b2.next_set_bit(bit.unwrap() + 1);
    //     assert_eq!(None, bit);
    //     // println!("{:?}", b.next_set_bit(0));
    // }

    // #[test]
    // fn test_bitset6() {
    //     let mut b = DBitset::new(1000);
    //     b.set(0);
    //     b.set(1);
    //     b.set(5);
    //     b.set(6);
    //     b.set(200);

    //     let mut res: Vec<String> = Vec::new();
    //     for bit in &b {
    //         res.push(bit.to_string());
    //     }
    //     assert_eq!("0 1 5 6 200", res.join(" "));
    // }
}