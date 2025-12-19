#![allow(dead_code, unused)]
use std::time::Instant;
use std::{error::Error, fs};
use std::collections::{VecDeque, HashSet};
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

fn beta_initialize_state(switches:&[Vec<usize>], joltage:Vec<usize>) -> usize {
    let mut seen = HashSet::new();
    let mut non_zeros = 0;
    for &j in &joltage {
        if j != 0 { non_zeros +=1; }
    }
    let mut queue = VecDeque::from([(joltage, non_zeros, 0)]);
    
    while let Some((joltage, non_zeros, steps)) = queue.pop_back() {
        for sw in switches {
            let mut update = joltage.clone();
            let mut update_non_zeros = non_zeros;
            for &jdx in sw {
                if update[jdx] == 0 { break; } else if update[jdx] == 1 {
                    update_non_zeros -= 1;
                }
                update[jdx] -= 1;
            }
            if update_non_zeros == 0 {
                return steps +1;
            }
            else if seen.insert(update.clone()) {
                queue.push_front((update, update_non_zeros, steps+1));
            }
            
        }
    }
    usize::MAX
}

fn beta_initialize_all_state(reqs:&[(Vec<Vec<usize>>, Vec<usize>)]) -> usize {
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
