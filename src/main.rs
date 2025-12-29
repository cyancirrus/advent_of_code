#![allow(dead_code, unused)]
use advent_of_code::parsers::day_ten::{parser_bits, parser_ints};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
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

// 3547 -> (1101)
// 2446
// 1223 -> (1001)
// 0222
// 0111 -> (0111)

fn beta_initialize_state(switches: &[u16], mut counters: Vec<u16>) -> usize {
    let mut non_zeros = 0;
    let mut steps = 0;
    let mut pos = 1;
    let n = counters.len();
    for &n in &counters {
        if n > 0 {
            non_zeros += 1;
        }
    }
    // println!("test {}", alpha_initialize_state(13, switches));
    while non_zeros > 0 {
        let mut odd = 0;
        let mut odds = vec![0; n];
        for i in 0..n {
            odd <<= 1;
            if counters[i] & 1 == 1 {
                odds[i] = 1;
                odd |= 1;
                counters[i] -= 1;
                if counters[i] == 0 {
                    non_zeros -= 1;
                }
            }
            counters[i] /= 2;
        }
        let tmp = alpha_initialize_state(odd, switches);
        steps += pos * alpha_initialize_state(odd, switches);
        pos <<= 1;
    }
    steps
}

fn beta_initialize_all_state(reqs: &[(u16, Vec<u16>, Vec<u16>)]) -> usize {
    let mut total = 0;
    let mut i = 0;
    for (_, switches, joltage) in reqs {
        total += beta_initialize_state(switches, joltage.to_vec());
        i += 1;
    }
    total
}

fn find_permutations(buttons: &[Vec<u16>], reqs: &[bool]) -> Vec<(Vec<u16>, Vec<usize>, u32)> {
    let n = reqs.len();
    let mut perms = vec![];
    for i in 0..1 << buttons.len() {
        let mut state = vec![false; n];
        let mut counts = vec![0; n];
        let mut indices = vec![];
        let mut presses = 0;
        let mut bit = i;
        let mut j = 0;
        while bit != 0 {
            if bit & 1 != 0 {
                presses += 1;
                indices.push(j);
                for &b in &buttons[j] {
                    state[b as usize] = !state[b as usize];
                    counts[b as usize] += 1;
                }
            }
            bit >>= 1;
            j += 1;
        }
        if reqs == state {
            perms.push((counts, indices, presses));
        }
    }
    perms
}

fn min_presses(buttons: &[Vec<u16>], joltage: &[u16], memo: &mut HashMap<Vec<u16>, u16>) -> u16 {
    if let Some(&result) = memo.get(joltage) {
        return result;
    }
    let mut n = joltage.len();
    if joltage.iter().all(|&j| j == 0) {
        return 0;
    }
    let mut reqs = vec![false; n];
    for (i, j) in joltage.iter().enumerate() {
        if j % 2 == 1 {
            reqs[i] = true;
        }
    }
    let perms = find_permutations(buttons, &reqs);
    let mut min = u16::MAX;
    for (counts, indices, presses) in perms {
        let mut remaining = vec![0; n];
        let mut infeasible = false;
        for idx in 0..n {
            if joltage[idx] < counts[idx] {
                infeasible = true;
                break;
            }
            remaining[idx] = (joltage[idx] - counts[idx]);
            if remaining[idx] % 2 == 1 {
                infeasible = true;
                break;
            } else {
                remaining[idx] /= 2;
            }
        }
        if infeasible {
            continue;
        }
        let recursive = min_presses(buttons, &remaining, memo);
        if recursive != u16::MAX {
            let candidate = 2 * recursive + presses as u16;
            min = min.min(candidate);
        }
    }
    memo.insert(joltage.to_vec(), min);
    min
}

fn gamma_min_presses(reqs: &[(Vec<Vec<u16>>, Vec<u16>)]) -> usize {
    let mut total = 0;
    println!("started");
    for (buttons, joltage) in reqs {
        let mut memo = HashMap::new();
        println!("loop");
        total += min_presses(buttons, joltage, &mut memo) as usize;
    }
    total
}

fn main() {
    // let reqs = vec![false, true, true, false];
    // // let buttons = vec![vec![3], vec![1,3], vec![2], vec![2,3], vec![0,2], vec![0,1]];
    // let buttons = vec![vec![1,3], vec![2,3], vec![0,1]];
    // println!("started");
    // let result = find_permutations(&buttons, &reqs);
    // println!(" Result {result:?}");
    // println!("-------------------------------------------------------------");
    // let reqs = parser_bits("./data/day_10.txt");
    // match reqs {
    //     Ok(mut p) => {
    //         let start = Instant::now();
    //         let result = alpha_initialize_all_state(&p);
    //         let time = start.elapsed();
    //         println!("Alpha found at {result:?}");
    //         println!("Alpha version: {} in {:?}", result, time);
    //         // let start = Instant::now();
    //         // let result = beta_find_max_rectangle(&mut p);
    //         // let time = start.elapsed();
    //         // println!("Beta version: {} in {:?}", result, time);
    //     }
    //     _ => {
    //         println!("Error in parsing");
    //     }
    // }
    println!("-------------------------------------------------------------");
    let reqs = parser_ints("./data/day_10.txt");
    match reqs {
        Ok(p) => {
            // println!("p {p:?}");
            let start = Instant::now();
            let result = gamma_min_presses(&p);
            let time = start.elapsed();
            println!("Beta found at {result:?}");
            println!("Gamma version: {} in {:?}", result, time);
        }
        _ => {
            println!("Error in parsing");
        }
    }
}
