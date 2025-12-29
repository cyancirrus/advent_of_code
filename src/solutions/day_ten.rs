#![allow(dead_code, unused)]
use crate::parsers::day_ten::{parser_bits, parser_ints};
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::time::Instant;
use std::{error::Error, fs};

fn alpha_initialize_state(objective: u16, switches: &[u16]) -> usize {
    let mut seen = HashSet::from([objective]);
    let mut queue = VecDeque::from([(objective, 0)]);
    while let Some((state, steps)) = queue.pop_front() {
        for s in switches {
            let update = s ^ state;
            if update == 0 {
                return steps + 1;
            } else if seen.insert(update) {
                queue.push_back((update, steps + 1));
            }
        }
    }
    return usize::MAX;
}

fn alpha_initialize_all_state(reqs: &[(u16, Vec<u16>, Vec<u16>)]) -> usize {
    let mut total = 0;
    for (objective, switches, _) in reqs {
        total += alpha_initialize_state(*objective, switches);
    }
    total
}

// fn main() {
//     println!("-------------------------------------------------------------");
//     let reqs = parser_bits("./data/day_10.txt");
//     match reqs {
//         Ok(mut p) => {
//             let start = Instant::now();
//             let result = alpha_initialize_all_state(&p);
//             let time = start.elapsed();
//             println!("Alpha found at {result:?}");
//             println!("Alpha version: {} in {:?}", result, time);
//             // let start = Instant::now();
//             // let result = beta_find_max_rectangle(&mut p);
//             // let time = start.elapsed();
//             // println!("Beta version: {} in {:?}", result, time);
//         }
//         _ => {
//             println!("Error in parsing");
//         }
//     }
//     println!("-------------------------------------------------------------");
//     let reqs = parser_bits("./data/day_10.txt");
//     match reqs {
//         Ok(p) => {
//             println!("p {p:?}");
//             let start = Instant::now();
//             let result = beta_initialize_all_state(&p);
//             let time = start.elapsed();
//             println!("Beta found at {result:?}");
//             println!("Beta version: {} in {:?}", result, time);
//         }
//         _ => {
//             println!("Error in parsing");
//         }
//     }
// }
