#![allow(dead_code, unused)]
use std::fmt;
use std::mem;
use std::{error::Error, fs};

pub fn parser(path:&str) -> Result<(Vec<usize>,Vec<Vec<bool>>), Box<dyn Error>> {
    let mut engine = vec![];
    let mut initials = vec![];

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(("Path {path:?} does not exist").into()),
    };
    let mut lines = content.lines();
    let mut length = 0;
    if let Some(first) = lines.next() {
        length = first.len();
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
        let mut splits = vec![false; length];
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

fn beta_tachyon_many_worlds(mut active:Vec<usize>, splits:&[Vec<bool>]) -> u64 {
    if splits.is_empty() || splits[0].is_empty() { return 0; }
    let length = splits[0].len();
    let mut count = 0;
    let mut next = vec![];
    let mut freqs= vec![0; length];
    let neighs = [!0, 1];
    for &i in active.iter() {
        freqs[i] = 1;
    }
    for l in splits {
        while let Some(x) = active.pop() {
            if !l[x] {
                // No branch push into next iteration
                next.push(x);
                continue;
            }
            for dx in neighs {
                // Handle boundaries
                let nx = x.wrapping_add(dx);
                if nx < length {
                    if freqs[nx] == 0 {
                        // Ensure queue is unique
                        next.push(nx);
                    }
                    freqs[nx] += freqs[x];
                }
            }
            freqs[x] = 0;
        }
        mem::swap(&mut active, &mut next);
    }
    for x in active {
        count += freqs[x]; 
    }
    count
}
fn main() {
    let tachyon = parser("./data/day_7.txt");
    match tachyon {
        Ok((mut a, s)) => {
            println!("Beta result {}", beta_tachyon_many_worlds(a.clone(), &s));
        }
        _ => {
            println!("Error in parsing");
        }
    }
}
