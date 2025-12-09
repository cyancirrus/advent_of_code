#![allow(dead_code, unused)]
use std::collections::BinaryHeap;
use std::mem;
use std::time::Instant;
use std::{error::Error, fs};

pub fn parser_ranges(path: &str) -> Result<Vec<(u64, u64)>, Box<dyn Error>> {
    let mut ranges = vec![]; 
    let content = match fs::read_to_string(path)  {
        Ok(c) => c,
        Err(e) => return Err(("Invalid path {path:?}").into()),
    };

    for line in content.lines() {
        let (left, right) = line.split_once("-").ok_or("deliminiter not found")?;
        match (left.parse(), right.parse()) {
            (Ok(l), Ok(r)) => ranges.push((l, r)),
            _ => return Err(("Invalid input in range data").into()),
        }
    }
    Ok(ranges)
}

pub fn parser_items(path: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    let mut items = vec![]; 
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(("Invalid path for items {path:?}").into()),
    };

    for line in content.lines() {
        match line.parse() {
            Ok(v) => items.push(v),
            Err(e) => return Err(("Invalid item found for items {e:?}").into()),
        }
    }
    Ok(items)
}

// could just run through the number of items
// could sort the input space and then remove the items
// could collapse the ranges into discrete sections
// sorting and then counting seems best
// want to see the hard problem prior to implementing binary heap
pub fn alpha_number_fresh(ranges:&[(u64,u64)], items:&[u64]) -> u64 {
    // Brute force 
    let mut count = 0;
    for &i in items {
        let mut found = false;
        for &(l, r) in ranges {
            if l <= i && i <= r {
                found = true;
                break;
            }
        }
        count += found as u64;
    }
    count
}


// Better version
// 1. sort by left
// don't really even have to merge can just sort and scan

pub fn gamma_number_fresh(ranges:&mut [(u64,u64)], items:&mut [u64]) -> u64 {
    let mut count = 0;
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    items.sort_by(|a, b| a.cmp(&b));
    let (mut i_idx, mut r_idx) = (0, 0);
    while r_idx < ranges.len() && i_idx < items.len() {
        if items[i_idx]  < ranges[r_idx].0 {
            i_idx +=1;
        } else if items[i_idx] <= ranges[r_idx].1 {
            count += 1;
            i_idx += 1;
        } else {
            r_idx += 1;
        }
    }
    count
}


pub fn beta_number_fresh(ranges:&mut [(u64, u64)]) -> u64 {
    if ranges.is_empty() { return 0; }
    let mut count = 0;
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let (mut left, mut right) = ranges[0];
    for &(l, r) in &ranges[1..] {
        if l <= right + 1 {
            right = right.max(r);
        } else {
            // inclusive ranges
            count += (right - left + 1);
            left = l;
            right = r;
        }
    }
    // ending is not finally parsed
    count + right - left + 1
}

// fn main() {
//     let ranges = parser_ranges("./data/day_5_ranges.txt");
//     let items = parser_items("./data/day_5_items.txt");
//     match (ranges, items) {
//         (Ok(mut r), Ok(mut i)) => {
//             println!("Alpha Number of fresh items {}", alpha_number_fresh(&r, &i));
//             println!("Gamma Number of fresh items {}", gamma_number_fresh(&mut r, &mut i));
//             // println!("Number of fresh items {}", beta_number_fresh(&mut r));
//         },
//         _ => {
//             println!("Unexpected error n alpha_number_fresh");
//         },
//     }
// }
