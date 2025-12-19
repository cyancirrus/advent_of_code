#![allow(dead_code, unused)]
use std::cmp::Ordering;
use std::time::Instant;
use std::{error::Error, fs};
use std::collections::{VecDeque, HashSet, BinaryHeap};
use advent_of_code::parsers::day_ten::{parser_bits, parser_ints};


fn alpha_initialize_state(objective:u16, switches:&[u16]) -> usize {
    let mut seen = HashSet::from([objective]);
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

#[derive(Eq, PartialEq, Clone, Debug)]
struct MinNode {
    estimate:u8,
    counters: Vec<u8>,
    non_zeros: u8,
    steps:usize,
}

impl Ord for MinNode {
    fn cmp(&self, other:&Self) -> Ordering {
        other.estimate.cmp(&self.estimate)
    }
}

impl PartialOrd for MinNode {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn beta_initialize_state(switches:&[Vec<u8>], counters:Vec<u8>) -> usize {
    let mut seen  = HashSet::new();
    let mut non_zeros = 0;
    let mut estimate = 0;
    for &num in &counters {
        if num != 0 { non_zeros +=1; }
        estimate += num;
    }
    let mut switch_count = switches.len();
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(MinNode{estimate, counters, non_zeros, steps: 0 });
    
    while let Some(node) = priority_queue.pop() {
        for sw in switches {
            let mut update = node.clone();
            update.steps += 1;
            for &jdx in sw {
                if update.counters[jdx as usize] == 0 { break; }
                else if update.counters[jdx as usize] == 1 {
                    update.non_zeros -=1;
                }
                update.counters[jdx as usize] -= 1;
                update.estimate -= 1;
            }
            if update.non_zeros == 0 {
                return update.steps;
            }
            else if seen.insert(update.counters.clone()) {
                priority_queue.push(update);
            }
        }
    }
    usize::MAX
}


fn beta_initialize_all_state(reqs:&[(Vec<Vec<u8>>, Vec<u8>)]) -> usize {
    let mut total = 0;
    let mut i = 0;
    for (switches, joltage) in reqs {
        println!("i {i:?}, joltage {joltage:?}");
        total += beta_initialize_state(switches, joltage.to_vec());
        i+=1;
    }
    total
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
    println!("-------------------------------------------------------------");
    let reqs = parser_ints("./data/day_10.txt");
    match reqs {
        Ok(p) => {
            println!("p {p:?}");
            let start = Instant::now();
            let result = beta_initialize_all_state(&p);
            let time = start.elapsed();
            println!("Beta found at {result:?}");
            println!("Beta version: {} in {:?}", result, time);
        }
        _ => {
            println!("Error in parsing");
        }
    }
}

