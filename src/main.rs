#![allow(dead_code, unused)]
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;
use std::{error::Error, fs};
use std::mem;

const NUMBER_SHAPES:usize = 6;
const N:usize = 9;

fn parser_shapes(path:&str) -> Result<Vec<Vec<bool>>, Box<dyn Error>> {
    let content = fs::read_to_string(path).map_err(|e| format!("Unable to read file. Returned with error\n{e:?}"))?;
    let mut shapes = Vec::with_capacity(NUMBER_SHAPES);
    let mut shape = Vec::with_capacity(N * N);
    let mut index = 0;
    for line in content.lines() {
        if line.is_empty() {
            shapes.push(shape.clone());
            shape.clear();
            index = 0;
        }
        for c in line.bytes() {
            match c {
                b'#' => shape.push(true),
                b'.' => shape.push(false),
                d if c.is_ascii_digit() => {
                    index = index * 10 + (d - b'0') as usize;
                },
                _ => {
                    // there's some unrelated characters
                }
            }
        }
    }
    Ok(shapes)
}

fn parser_fills(path:&str) -> Result<Vec<(usize, usize, Vec<usize>)>, Box<dyn Error>> {
    let content = fs::read_to_string(path).map_err(|e| format!("Unable to read file. Returned with error\n{e:?}"))?;
    let mut fills = Vec::new();
    for line in content.lines() {
        let mut requirement = (usize::MAX, usize::MAX, Vec::new());
        let (dim_string, indices_string) = line.split_once(":").ok_or("Unable to split the dimension requirement")?;
        let mut num = 0;
        for c in dim_string.bytes() {
            match c {
                b'x' => {
                    requirement.0 = num;
                    num = 0;
                },
                d if c.is_ascii_digit() => {
                    num = num * 10 + (d - b'0') as usize;
                },
                _ => {
                    return Err(format!("Unable to parse the requirments.").into());
                }
            }
        }
        requirement.1 = num;
        num = 0;
        // for c in indices_string.bytes() {
        //     match c {
        //         d if 
        //     }
        // }
        let indices = indices_string.split_whitespace();
        for idx in indices {
            match idx.parse() {
                Ok(i) => requirement.2.push(i),
                Err(e) => return Err(format!("Unable to parse indices.\n{e:?}").into()),
            }
        }
        fills.push(requirement);
    }
    Ok(fills)
}




fn main() {
    let shapes = "./data/day_12_sample_shapes.txt";
    let fills = "./data/day_12_sample_fills.txt";
    let data = parser_shapes(shapes);
    let fills = parser_fills(fills);
    match (data, fills) {
        (Ok(d), Ok(f)) => {
            println!("data {d:?}");
            println!("fills {f:?}");
            // let start = Instant::now();
            // let result = gamma_find_number_paths(&c);
            // let time = start.elapsed();
            // println!("Gamma version: {} in {:?}", result, time);
            // let start = Instant::now();
            // let result = beta_find_number_paths(&mut c);
            // let time = start.elapsed();
            // println!("Beta version: {:?} in {:?}", result, time);
            // let start = Instant::now();
            // let result = beta_find_number_paths(&mut c);
            // let time = start.elapsed();
            // println!("Beta version: {} in {:?}", result, time);
        }
        _ => {
            println!("Error in parsing");
        }
    }
}
