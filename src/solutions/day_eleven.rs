#![allow(dead_code, unused)]
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;
use std::{error::Error, fs};
use std::mem;

pub fn alpha_find_number_paths(node_map: &HashMap<String, Vec<String>>) -> usize {
    // should work but could probably do something smarter the seen hashmap is subpar
    let mut queue: VecDeque<(&str, HashSet<String>)> = VecDeque::new();
    let mut path_count = 0;
    let start = "you";
    queue.push_back((start, HashSet::new()));
    while let Some((path, seen)) = queue.pop_front() {
        if let Some(neighs) = node_map.get(path) {
            for n in neighs {
                if n == "out" {
                    path_count += 1;
                    continue;
                } else if !seen.contains(n) {
                    let mut new_seen = seen.clone();
                    new_seen.insert(n.to_string());
                    queue.push_back((n, new_seen));
                }
            }
        }
    }
    path_count
}


pub fn gamma_find_number_paths(node_map: &HashMap<String, Vec<String>>) -> usize {
    let mut curr: HashMap<&str, usize> = HashMap::new();
    let mut prev: HashMap<&str, usize> = HashMap::new();

    let start = "you";
    let end = "out";
    let mut path_count = 0;
    prev.insert(start, 1);
    while !prev.is_empty() {
        curr.clear();
        for (k, v) in node_map {
            let k = k.as_str();
            if let Some(&ck) = prev.get(k) {
                if ck == 0 { continue; }
                for n in v {
                    let n = n.as_str();
                    if n == end {
                        path_count += ck;
                    } else {
                        *curr.entry(n).or_insert(0) += ck;
                    }
                }
            }
        }
        mem::swap(&mut curr, &mut prev);
    }
    path_count
}



fn beta_find_number_paths(node_map: &HashMap<String, Vec<String>>) -> usize {
    let mut curr: HashMap<&str, (usize, usize, usize, usize)> = HashMap::new();
    let mut prev: HashMap<&str, (usize, usize, usize, usize)> = HashMap::new();

    let start = "svr";
    let end = "out";
    let must_pass_1 = "fft";
    let must_pass_2 = "dac";
    prev.insert(start, (1, 0, 0, 0 ));
    let mut result = 0;
    while !prev.is_empty() {
        // println!("prev {prev:?}");
        curr.clear();
        for (k, v) in node_map {
            let k = k.as_str();
            if let Some(&(ck, ck_mp1, ck_mp2, ck_all)) = prev.get(k) {
                if ck == 0 && ck_mp1 == 0 && ck_mp2 == 0 && ck_all == 0 { continue; }
                for n in v {
                    let n = n.as_str();
                    let entry = curr.entry(n).or_insert((0,0,0,0));
                    let (cn, cn_mp1, cn_mp2, cn_all) = entry;
                    if n == must_pass_1 {
                        *cn_mp1 += ck + ck_mp1;
                        *cn_all += ck_mp2 + ck_all;
                    } else if n == must_pass_2 {
                        *cn_mp2 += ck + ck_mp2;
                        *cn_all += ck_mp1 + ck_all;
                    } else {
                        *cn += ck;
                        *cn_mp1 += ck_mp1;
                        *cn_mp2 += ck_mp2;
                        *cn_all += ck_all;
                    }
                }
            }
        }
        if let Some(&(_,_,_,r)) = curr.get(end) {
            result += r;
        }
        mem::swap(&mut curr, &mut prev);
    }
    result
}


// fn main() {
//     println!("hello world");
//     let nmap = parser("./data/day_11.txt");
//     match nmap {
//         Ok(mut c) => {
//             let start = Instant::now();
//             let result = gamma_find_number_paths(&c);
//             let time = start.elapsed();
//             println!("Gamma version: {} in {:?}", result, time);
//             let start = Instant::now();
//             let result = beta_find_number_paths(&mut c);
//             let time = start.elapsed();
//             println!("Beta version: {:?} in {:?}", result, time);
//             // let start = Instant::now();
//             // let result = beta_find_number_paths(&mut c);
//             // let time = start.elapsed();
//             // println!("Beta version: {} in {:?}", result, time);
//         }
//         _ => {
//             println!("Error in parsing");
//         }
//     }
// }
