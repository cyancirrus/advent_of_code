#![allow(dead_code, unused)]
use std::fmt;
use std::mem;
use std::{error::Error, fs};

const SIZE:usize = 256;

pub fn parser(path:&str) -> Result<(Vec<usize>,Vec<Vec<bool>>), Box<dyn Error>> {
    let mut engine = vec![];
    let mut initials = vec![];

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(("Path {path:?} does not exist").into()),
    };
    let mut lines = content.lines();
    if let Some(first) = lines.next() {
        for (i, b) in first.bytes().enumerate() {
            match b {
                b'S' => {
                    initials.push(i);
                    break;
                },
                b'.' => continue,
                _ => return Err(("Unexpected symbol {b}").into()),
            }
        }
    }
    for l in lines.skip(1).step_by(2) {
        let mut splits = vec![false; SIZE];
        for (i, b) in l.bytes().enumerate() {
            match b {
                b'^' => {
                    splits[i] = true;
                },
                b'.' => continue,
                _ => return Err(("Unexpected symbol {b}").into()),
            }
        }
        engine.push(splits);
    }
     Ok((initials, engine))
}

fn beta_tachyon_many_worlds(active:&mut Vec<usize>, splits:&[Vec<bool>]) -> u32 {
    if splits.is_empty() || splits[0].is_empty() { return 0; }
    let n = splits[0].len();
    let mut count = 0;
    let mut freqs= vec![0; SIZE];
    let neighs = [!0, 1];
    for &i in active.iter() {
        freqs[i] = 1;
    }
    for l in splits {
        for &x in active.iter() {
            if freqs[x] ==  0 { continue; }
            for n in neighs {
                let nx = x.wrapping_add(n);
                if nx < n {
                    freqs[nx] += freqs[x];
                    freqs[x] = 0;
                }
            }
        }
    }
    for &x in active.iter() {
        count += freqs[x]; 
    }
    count
}


fn main() {
    // let tachyon = parser_demo("./data/day_7.txt");
    // match tachyon {
    //     Ok(t) => {
    //         println!("Alpha demo result {}", alpha_understand_tachyon_demo(&t));
    //     }
    //     _ => {
    //         println!("Error demo in parsing");
    //     }
    // }
    // let tachyon = parser("./data/day_7.txt");
    // match tachyon {
    //     Ok(t) => {
    //         println!("Alpha result {}", alpha_understand_tachyon(&t));
    //         println!("Should be Alpha result 1656");
    //         // alpha_understand_tachyon_debug(&t);
    //     }
    //     _ => {
    //         println!("Error in parsing");
    //     }
    // }
}
