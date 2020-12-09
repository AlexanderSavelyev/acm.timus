
// use std::cmp;

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
        // println!("find word idx {}", word_idx);

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
        if map_idx > 0 && map_idx < self.words_map.len() - 1 {
            let next = map_idx + 1;
            let prev = map_idx - 1;
            if self.words_map[prev].num_bits == 0 && self.words_map[next].num_bits == 0 {
                self.words_map[prev].reference = self.words_map[next].reference;
                self.words_map.remove(map_idx);
                self.words_map.remove(map_idx);
            } else if self.words_map[prev].num_bits == 0 && self.words_map[next].num_bits > 0 {
                self.words_map[prev].reference = word_idx;
                self.words_map.remove(map_idx);
            } else if self.words_map[prev].num_bits > 0 && self.words_map[next].num_bits == 0 {
                self.words_map[next].position = word_idx;
                self.words_map.remove(map_idx);
            } else {
                self.words_map[map_idx].reference = word_idx;
            }
        } else if map_idx > 0 {
            let prev = map_idx - 1;
            if self.words_map[prev].num_bits == 0 {
                self.words_map[prev].reference = word_idx;
                self.words_map.remove(map_idx);
            } else {
                self.words_map[map_idx].reference = word_idx;
            }
        } else if map_idx < self.words_map.len() - 1 {
            let next = map_idx + 1;
            if self.words_map[next].num_bits == 0 {
                self.words_map[next].position = word_idx;
                self.words_map.remove(map_idx);
            } else {
                self.words_map[map_idx].reference = word_idx;
            }
        } else {
            self.words_map[map_idx].reference = word_idx;
        }
    }
    fn set(&mut self, bit_idx: usize) -> bool {
        // println!("set {}", bit_idx);
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
        // println!("reset {}", bit_idx);
        let word_idx = SparseBitset::word_index(bit_idx);
        let map_idx = self.find_map_idx(word_idx as u32);

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

    fn get(&self, bit_idx: usize) -> bool {
        let word_idx = SparseBitset::word_index(bit_idx);
        let map_idx = self.find_map_idx(word_idx as u32);

        if self.words_map[map_idx].num_bits == 0 {
            return false;
        }
        let mut bit = bit_idx;
        bit -= word_idx << ADDRESS_BITS_PER_WORD;
        let word = self.words_map[map_idx].reference as usize;

        (self.words[word] & (1u64 << bit)) != 0
    }

    // fn and_with(&mut self, set: &SparseBitset) {
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
    // fn and_not_with(&mut self, set: &SparseBitset) {
    //     let w_min = cmp::min(self.words_in_use, set.words_in_use);
    //     for i in 0..w_min {
    //         self.words[i] &= !set.words[i];
    //     }
    //     self.recalculate_words_in_use();
    // }
    fn is_subset_of(&self, set: &SparseBitset) -> bool {
        let mut map_idx = 0_usize;
        for wm in &self.words_map {
            if wm.num_bits == 0 {
                continue;
            }
            let word_idx = wm.position;
            // println!("start {}", word_idx);
            loop {
                let set_wm = &set.words_map[map_idx];
                if set_wm.position <= word_idx {
                    if set_wm.num_bits == 0 && word_idx <= set_wm.reference {
                        // println!("return false num bits 0 {}", word_idx);
                        return false;
                    }
                    if set_wm.position == word_idx && set_wm.num_bits > 0{
                        // println!("check set {}", word_idx);
                        if (self.words[wm.reference as usize] & (!set.words[set_wm.reference as usize])) != 0 {
                            // println!("return false & {}", word_idx);
                            return false;
                        }
                        break;
                    }
                }
                map_idx += 1;
                if map_idx >= set.words_map.len() {
                    // println!("return false len {}", map_idx);
                    return false;
                }
            }
        }
        return true;
    }
    // fn or_with(&mut self, set: &SparseBitset) -> bool {
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

    fn next_set_bit(&self, from_index: usize, from_map: usize) -> Option<(usize, usize)> {
        let mut from_idx = from_index;
        let mut map_idx = from_map;
        let from_word = SparseBitset::word_index(from_idx);
        let mut word = 0_u64;
        loop {
            if map_idx >= self.words_map.len() {
                return None;
            }
            let wm = &self.words_map[map_idx];
            if wm.num_bits > 0 {
                if wm.position == (from_word as u32) {
                    from_idx -= (from_word << ADDRESS_BITS_PER_WORD);
                    word = self.words[wm.reference as usize] & (WORD_MASK << from_idx);
                } else {
                    word = self.words[wm.reference as usize];
                }
            }

            if word != 0 {
                let bit = (wm.position as usize) << ADDRESS_BITS_PER_WORD;
                let lbit = SparseBitset::least_significant_bit_position(word);
                
                return Some((bit + lbit.unwrap(), map_idx));
            }

            map_idx += 1;
        }
    }

    // let mut bit = cell.next_set_bit(0);
    // while bit.is_some() {
    //     let b = bit.unwrap();
    //     bit = cell.next_set_bit(b + 1);
    // }
    //
}




impl<'a> IntoIterator for &'a SparseBitset {
    type Item = usize;
    type IntoIter = SBitsetIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SBitsetIterator {
            bitset: self,
            index: None,
            map_idx: 0,
        }
    }
}

struct SBitsetIterator<'a> {
    bitset: &'a SparseBitset,
    index: Option<usize>,
    map_idx: usize,
}

impl<'a> Iterator for SBitsetIterator<'a> {
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
        match self.bitset.next_set_bit(start_idx, self.map_idx) {
            Some((bit, from_map)) => {
                self.index.replace(bit);
                self.map_idx = from_map;
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
    // rustc --test sparse_bitset.rs; ./d_bitset --nocapture
    use SparseBitset;

    #[test]
    fn test_bitset1() {
        let mut b = SparseBitset::new(1000);
        // assert_eq!(false, b.get(65));
        b.set(65);
        assert_eq!(3, b.words_map.len());
        assert_eq!(1, b.words.len());
        assert_eq!(1, b.num_bits);
        // println!("words_map {:?}", b.words_map);
        // println!("words {:?}", b.words);
        // println!("num_bits {:?}", b.num_bits);
        b.set(64);
        assert_eq!(3, b.words_map.len());
        assert_eq!(1, b.words.len());
        assert_eq!(2, b.num_bits);
        b.set(15);
        assert_eq!(3, b.words_map.len());
        assert_eq!(2, b.words.len());
        assert_eq!(3, b.num_bits);
        b.set(70);
        assert_eq!(3, b.words_map.len());
        assert_eq!(2, b.words.len());
        assert_eq!(4, b.num_bits);
        b.set(200);
        assert_eq!(5, b.words_map.len());
        assert_eq!(3, b.words.len());
        assert_eq!(5, b.num_bits);
        // assert_eq!(true, b.get(65));
    }

    #[test]
    fn test_bitset2() {
        let mut b = SparseBitset::new(1000);
        b.set(925);
        assert_eq!(true, b.get(925));
        assert_eq!(false, b.get(800));
        assert_eq!(false, b.get(926));
    }
    #[test]
    fn test_bitset3() {
        let mut b1 = SparseBitset::new(1000);
        let mut b2 = SparseBitset::new(1000);
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
        let mut b = SparseBitset::new(1000);
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

    // #[test]
    // fn test_bitset5() {
    //     let mut b = SparseBitset::new(1024);
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
    //     let mut b2 = SparseBitset::new(1024);
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
    //     let mut b = SparseBitset::new(1000);
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