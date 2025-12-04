#![allow(dead_code, unused)]
use advent_of_code::solutions::day_three::{parser, alpha_max_voltage, beta_max_voltage};
use std::collections::VecDeque;
use std::{error::Error, fs};
use std::mem;

const BANK:usize = 100;
const BASE:usize = 12;


fn main() {
    let mut num_parse = parser("./data/day_3.txt");
    match num_parse {
        Ok(bats) => {
            println!("Alpha version {}", alpha_max_voltage(&bats));
            println!("Beta version {}", beta_max_voltage(&bats));
        },
        Err(e) => {
            println!("Unsuccessful error {:?}", e);
        }
    }
    
}
