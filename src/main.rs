#![allow(dead_code, unused)]
use advent_of_code::parsers::day_ten::{parser_bits, parser_ints};
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::time::Instant;
use std::{error::Error, fs};

fn main() {
    println!("hello world");
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
}
