use bitvec::prelude::*;
use std::{collections::HashMap, ops::BitAnd};

// struct Trie {
//     map: HashMap<char, Trie>,
//     val: Option<char>,
// }

// impl Trie {
//     pub fn default() -> Trie {
//         Trie {
//             map: HashMap::new(),
//             val: None,
//         }
//     }

//     pub fn insert(&mut self, s: &str, val: char) {
//         let mut curr = self;
//         for c in s.chars() {
//             curr = curr.map.entry(c).or_insert(Trie::default());
//         }
//         curr.val = Some(val);
//     }

//     pub fn find(&self, s: &str) -> Option<char> {
//         let mut curr = self;
//         for c in s.chars() {
//             if !curr.map.contains_key(&c) {
//                 return None;
//             }
//             curr = curr.map.get(&c).unwrap();
//         }
//         curr.val
//     }
// }

struct SevenSegment {
    map: HashMap<BitArray, i32>,
}

impl SevenSegment {
    fn to_bits(s: &str) -> Option<BitArray> {
        let mut v = bitarr!(0; 7);
        for c in s.chars() {
            match c {
                'a'..='g' => {
                    v.set(c as usize - 'a' as usize, true);
                }
                _ => return None,
            }
        }
        return Some(v);
    }
    pub fn new(line: &str) -> Self {
        let mut graph = vec![Vec::new(); 8];
        line.split(' ')
            .filter_map(SevenSegment::to_bits)
            .for_each(|s| graph[s.count_ones()].push(s));

        // create trie
        let mut map: HashMap<BitArray, i32> = HashMap::new();
        let mut lookup: Vec<BitArray<_, _>> = vec![bitarr!(0; 7); 10];

        // 1 => always size 2
        for s in &graph[2] {
            map.insert(*s, 1);
            lookup[1] = *s;
        }
        // 7 => always size 3
        for s in &graph[3] {
            map.insert(*s, 7);
            lookup[7] = *s;
        }
        // 4 => always size 4
        for s in &graph[4] {
            map.insert(*s, 4);
            lookup[4] = *s;
        }
        // 8 => always size 7
        for s in &graph[7] {
            map.insert(*s, 8);
        }
        // 9,0,6,
        for s in &graph[6] {
            if s.bitand(lookup[4]) == lookup[4] {
                map.insert(*s, 9);
                lookup[9] = *s;
            } else if s.bitand(lookup[7]) == lookup[7] {
                map.insert(*s, 0);
                lookup[0] = *s;
            } else {
                map.insert(*s, 6);
                lookup[6] = *s;
            }
        }
        // 3,2,5
        for s in &graph[5] {
            if s.bitand(lookup[1]) == lookup[1] {
                map.insert(*s, 3);
                lookup[3] = *s;
            } else if lookup[4].bitand(*s).count_ones() == 3 {
                map.insert(*s, 5);
                lookup[5] = *s;
            } else {
                map.insert(*s, 2);
                lookup[2] = *s;
            }
        }

        Self { map: map }
    }

    pub fn decode(&self, s: &str) -> i32 {
        let mut ret = 0;
        s.split(' ').for_each(|s| {
            if let Some(bits) = Self::to_bits(s) {
                if self.map.contains_key(&bits) {
                    ret *= 10;
                    ret += self.map.get(&bits).unwrap();
                }
            }
        });
        ret
    }
}

fn main() {
    let sum: i32 = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let ss = SevenSegment::new(l);
            let num = ss.decode(l.split_once(" | ").unwrap().1);
            num
        })
        .sum();

    print!("{}", sum);
}
