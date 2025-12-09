#![allow(dead_code, unused)]
use std::fmt;
use std::mem;
use std::{error::Error, fs};

const PRECISION: usize = 64;

#[derive(Copy, Clone)]
pub struct BitMask {
    l: u64,
    m: u64,
    r: u64,
}

pub struct Tachyon {
    initial: BitMask,
    engine: Vec<BitMask>,
}

impl fmt::Debug for BitMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // helper to print a single u64 as bits, MSB on the left
        fn print_bits_u64(mut n: u64, bits: usize) -> String {
            let mut s = String::with_capacity(bits);
            for i in (0..bits).rev() {
                let bit = (n >> i) & 1;
                s.push(if bit == 1 { '|' } else { '.' });
            }
            s
        }
        // Concatenate the chunks
        let row = format!(
            "{}{}{}",
            print_bits_u64(self.l, PRECISION),
            print_bits_u64(self.m, PRECISION),
            print_bits_u64(self.r, PRECISION)
        );

        write!(f, "{}", row)
    }
}

impl BitMask {
    pub fn format_bits(&self, zero: char, one: char) -> String {
        fn bits_to_string(mut n: u64, bits: usize, zero: char, one: char) -> String {
            let mut s = String::with_capacity(bits);
            for i in (0..bits).rev() {
                s.push(if (n >> i) & 1 == 1 { one } else { zero });
            }
            s
        }

        let mut row = String::new();
        row.push_str(&bits_to_string(self.l, PRECISION, zero, one));
        row.push_str(&bits_to_string(self.m, PRECISION, zero, one));
        row.push_str(&bits_to_string(self.r, PRECISION, zero, one));
        row
    }
    pub fn initialize(&mut self, offset: usize) {
        let container = offset / PRECISION;
        let precision = offset % PRECISION;
        match container {
            0 => {
                self.r |= 1 << precision;
            }
            1 => {
                self.m |= 1 << precision;
            }
            2 => {
                self.l |= 1 << precision;
            }
            _ => {
                debug_assert!(false, "this shouldn't happen incorrect precision");
            }
        }
    }
    pub fn count_ones(&self) -> u32 {
        self.l.count_ones() + self.m.count_ones() + self.r.count_ones()
    }
    pub fn left_shift(bm: &BitMask) -> BitMask {
        let mut new_bm = BitMask {
            l: bm.l << 1,
            m: bm.m << 1,
            r: bm.r << 1,
        };
        new_bm.l |= (bm.m & (1 << PRECISION - 1)) >> PRECISION - 1;
        new_bm.m |= (bm.r & (1 << PRECISION - 1)) >> PRECISION - 1;
        new_bm
    }
    pub fn right_shift(bm: &BitMask) -> BitMask {
        let mut new_bm = BitMask {
            l: bm.l >> 1,
            m: bm.m >> 1,
            r: bm.r >> 1,
        };
        new_bm.m |= (bm.l & 1) << (PRECISION - 1);
        new_bm.r |= (bm.m & 1) << (PRECISION - 1);
        new_bm
    }
    pub fn or_equal(&mut self, bm: &BitMask) {
        self.l |= bm.l;
        self.m |= bm.m;
        self.r |= bm.r;
    }
    pub fn and(a: &BitMask, b: &BitMask) -> BitMask {
        BitMask {
            l: a.l & b.l,
            m: a.m & b.m,
            r: a.r & b.r,
        }
    }
    pub fn and_equal(&mut self, bm: &BitMask) {
        self.l &= bm.l;
        self.m &= bm.m;
        self.r &= bm.r;
    }
    pub fn nand_equal(&mut self, bm: &BitMask) {
        self.l &= !bm.l;
        self.m &= !bm.m;
        self.r &= !bm.r;
    }
}

#[derive(Debug)]
pub struct TachyonDemo {
    initial: u64,
    engine: Vec<u64>,
}

pub fn parser_demo(path: &str) -> Result<TachyonDemo, Box<dyn Error>> {
    let mut initial = 1;
    let mut engine = vec![];

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(("Path {path:?} does not exist").into()),
    };
    let mut lines = content.lines();
    if let Some(first) = lines.next() {
        for b in first.bytes().rev() {
            match b {
                b'S' => break,
                b'.' => initial <<= 1,
                _ => return Err(("Unexpected symbol {b}").into()),
            }
        }
    }
    for l in lines.skip(1).step_by(2) {
        let mut bits = 0;
        for (p, b) in l.bytes().rev().enumerate() {
            match b {
                b'^' => bits |= 1 << p,
                b'.' => continue,
                _ => return Err(("Unexpected symbol {b}").into()),
            }
        }
        engine.push(bits);
    }
    let tachyon = TachyonDemo { initial, engine };
    Ok(tachyon)
}

pub fn parser(path: &str) -> Result<Tachyon, Box<dyn Error>> {
    let mut initial_offset = 0;
    let mut engine = vec![];

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(("Path {path:?} does not exist").into()),
    };
    let mut lines = content.lines();
    if let Some(first) = lines.next() {
        for (p, b) in first.bytes().rev().enumerate() {
            match b {
                b'S' => break,
                b'.' => initial_offset += 1,
                _ => return Err(("Unexpected symbol {b}").into()),
            }
        }
    }

    let mut initial = BitMask { l: 0, m: 0, r: 0 };
    initial.initialize(initial_offset);
    let mut counter = 0;

    for l in lines.skip(1).step_by(2) {
        let mut bitmask = BitMask { l: 0, m: 0, r: 0 };
        for (p, b) in l.bytes().rev().enumerate() {
            match b {
                b'^' => {
                    bitmask.initialize(p);
                    counter += 1;
                }
                b'.' => continue,
                _ => return Err(("Unexpected symbol {b}").into()),
            }
        }
        engine.push(bitmask);
    }
    let tachyon = Tachyon { initial, engine };
    Ok(tachyon)
}

pub fn alpha_understand_tachyon(tachyon: &Tachyon) -> u32 {
    let mut count = 0;
    let mut curr = tachyon.initial;
    for layer in &tachyon.engine {
        let splits = BitMask::and(&curr, &layer);
        curr.or_equal(&BitMask::left_shift(&splits));
        curr.or_equal(&BitMask::right_shift(&splits));
        curr.nand_equal(&splits);
        count += splits.count_ones();
        // println!("{}", layer.format_bits('.', '^'));
        // println!("{}", curr.format_bits('.', '.'));
    }
    count
}

pub fn beta_understand_tachyon(tachyon: &Tachyon) -> u32 {
    // Can't use the bit techniques very well
    let mut count = 0;
    let mut curr = tachyon.initial;
    for layer in &tachyon.engine {
        let splits = BitMask::and(&curr, &layer);
        curr.or_equal(&BitMask::left_shift(&splits));
        curr.or_equal(&BitMask::right_shift(&splits));
        curr.nand_equal(&splits);
        count += splits.count_ones();
        // println!("{}", layer.format_bits('.', '^'));
        // println!("{}", curr.format_bits('.', '.'));
    }
    count
}

pub fn alpha_understand_tachyon_debug(tachyon: &Tachyon) {
    let mut count = 0;
    let mut curr = tachyon.initial;
    for layer in &tachyon.engine {
        println!("{layer:?}");
    }
}

// fn main() {
//     // let tachyon = parser_demo("./data/day_7.txt");
//     // match tachyon {
//     //     Ok(t) => {
//     //         println!("Alpha demo result {}", alpha_understand_tachyon_demo(&t));
//     //     }
//     //     _ => {
//     //         println!("Error demo in parsing");
//     //     }
//     // }
//     let tachyon = parser("./data/day_7.txt");
//     match tachyon {
//         Ok(t) => {
//             println!("Alpha result {}", alpha_understand_tachyon(&t));
//             println!("Should be Alpha result 1656");
//             // alpha_understand_tachyon_debug(&t);
//         }
//         _ => {
//             println!("Error in parsing");
//         }
//     }
// }
