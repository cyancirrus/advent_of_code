#![allow(dead_code, unused)]
use std::{error::Error, fs};

const NUMBER_SHAPES: usize = 6;
const N: usize = 3;

pub fn parser_shapes(path: &str) -> Result<Vec<Vec<bool>>, Box<dyn Error>> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Unable to read file. Returned with error\n{e:?}"))?;
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
                }
                _ => {
                    // there's some unrelated characters
                }
            }
        }
    }
    Ok(shapes)
}

pub fn parser_fills(path: &str) -> Result<Vec<(usize, usize, Vec<usize>)>, Box<dyn Error>> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Unable to read file. Returned with error\n{e:?}"))?;
    let mut fills = Vec::new();
    for line in content.lines() {
        let mut requirement = (usize::MAX, usize::MAX, Vec::new());
        let (dim_string, indices_string) = line
            .split_once(":")
            .ok_or("Unable to split the dimension requirement")?;
        let mut num = 0;
        for c in dim_string.bytes() {
            match c {
                b'x' => {
                    requirement.0 = num;
                    num = 0;
                }
                d if d.is_ascii_digit() => {
                    num = num * 10 + (d - b'0') as usize;
                }
                _ => {
                    return Err(format!("Unable to parse the requirments.").into());
                }
            }
        }
        requirement.1 = num;
        num = 0;
        for c in indices_string.bytes().skip(1) {
            match c {
                d if d.is_ascii_digit() => {
                    num = num * 10 + (d - b'0') as usize;
                }
                b' ' => {
                    requirement.2.push(num);
                    num = 0;
                }
                _ => return Err(format!("Unable to parse indices.").into()),
            }
        }
        requirement.2.push(num);
        fills.push(requirement);
    }
    Ok(fills)
}
