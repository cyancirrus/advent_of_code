#![allow(dead_code, unused)]
use std::time::Instant;
use std::{error::Error, fs};
use std::collections::{VecDeque, HashSet};
use advent_of_code::parsers::day_ten::parser_bits;


fn alpha_initialize_state(objective:u16, switches:&[u16]) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([(objective, 0)]);
    while let Some((state, steps)) = queue.pop_front() {
        for s in switches {
            let update = s ^ state;
            if update == 0 { return steps + 1;
            } else if seen.insert(update) {
                queue.push_back((update,steps + 1));
            }
        }
    }
    return usize::MAX
}

fn alpha_initialize_all_state(reqs:&[(u16, Vec<u16>, Vec<u16>)]) -> usize {
    let mut total = 0;
    for (objective, switches, _) in reqs {
        total += alpha_initialize_state(*objective, switches);
    }
    total
}

fn beta_intialize_state(switches:&[u16], joltage:Vec<u16>) {
    let mut seen = HashSet::new();
    seen.insert(vec![1,2,3]);
}

fn main() {
    println!("-------------------------------------------------------------");
    let reqs = parser_bits("./data/day_10.txt");
    match reqs {
        Ok(mut p) => {
            let start = Instant::now();
            let result = alpha_initialize_all_state(&p);
            let time = start.elapsed();
            println!("Alpha found at {result:?}");
            println!("Alpha version: {} in {:?}", result, time);
            // let start = Instant::now();
            // let result = beta_find_max_rectangle(&mut p);
            // let time = start.elapsed();
            // println!("Beta version: {} in {:?}", result, time);
        }
        _ => {
            println!("Error in parsing");
        }
    }
}
