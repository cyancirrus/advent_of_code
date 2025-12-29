#![allow(dead_code, unused)]
use advent_of_code::parsers::day_twelve::{parser_fills, parser_shapes};
use advent_of_code::solutions::day_twelve_rotations::{
    fill_rotate_0, fill_rotate_1, fill_rotate_2, fill_rotate_3, unfill_rotate_0, unfill_rotate_1,
    unfill_rotate_2, unfill_rotate_3,
};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem;
use std::time::Instant;
use std::{error::Error, fs};

const NUMBER_SHAPES: usize = 6;
const N: usize = 3;

fn print_fill(data: &[bool], m: usize, n: usize) {
    for i in 0..m {
        let mut s = String::new();
        for j in 0..n {
            if data[i * n + j] {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        println!("{s:?}");
    }
}

fn fill_shape(
    space: &mut [bool],
    shapes: &[Vec<bool>],
    reqs: &mut [usize],
    mut idx: usize,
    m: usize,
    n: usize,
) -> bool {
    loop {
        if idx == reqs.len() {
            return true;
        } else if reqs[idx] == 0 {
            idx += 1;
        } else {
            break;
        }
    }
    let shape = &shapes[idx];

    for i in 0..m {
        for j in 0..n {
            if fill_rotate_0(space, shape, m, n, i, j) {
                reqs[idx] -= 1;
                if fill_shape(space, shapes, reqs, idx, m, n) {
                    return true;
                } else {
                    unfill_rotate_0(space, shape, m, n, i, j);
                }
                reqs[idx] += 1;
            }
            if fill_rotate_1(space, shape, m, n, i, j) {
                reqs[idx] -= 1;
                if fill_shape(space, shapes, reqs, idx, m, n) {
                    return true;
                } else {
                    unfill_rotate_1(space, shape, m, n, i, j);
                }
                reqs[idx] += 1;
            }
            if fill_rotate_2(space, shape, m, n, i, j) {
                reqs[idx] -= 1;
                if fill_shape(space, shapes, reqs, idx, m, n) {
                    return true;
                } else {
                    unfill_rotate_2(space, shape, m, n, i, j);
                }
                reqs[idx] += 1;
            }
            if fill_rotate_3(space, shape, m, n, i, j) {
                reqs[idx] -= 1;
                if fill_shape(space, shapes, reqs, idx, m, n) {
                    return true;
                } else {
                    unfill_rotate_3(space, shape, m, n, i, j);
                }
                reqs[idx] += 1;
            }
        }
    }
    false
}

fn alpha_fill_shapes(
    requirements: &mut [(usize, usize, Vec<usize>)],
    shapes: &[Vec<bool>],
) -> usize {
    let mut solveables = 0;
    for (m, n, reqs) in requirements.iter_mut() {
        let mut space = vec![false; *m * *n];
        solveables += fill_shape(&mut space, shapes, reqs, 0, *m, *n) as usize;
    }
    solveables
}

fn main() {
    // let shapes = "./data/day_12_sample_shapes.txt";
    // let fills = "./data/day_12_sample_fills.txt";
    let shapes = "./data/day_12_shapes.txt";
    let fills = "./data/day_12_fills.txt";
    let shapes = parser_shapes(shapes);
    let fills = parser_fills(fills);
    // let test_shape = vec![true, true, true, true, false, false, true, true, true];
    // let (m, n) = (4, 12);
    // let (x, y) = (1, 1);
    // print_fill(&test_shape, 3, 3);

    //     println!("-------------");
    //     let mut test_space = vec![false; m * n];
    //     fill_rotate_0(&mut test_space, &test_shape, m, n, x, y);
    //     print_fill(&test_space, m, n);

    //     println!("-------------");
    //     let mut test_space = vec![false; m * n];
    //     fill_rotate_1(&mut test_space, &test_shape, m, n, x, y);
    //     print_fill(&test_space, m, n);

    //     println!("-------------");
    //     let mut test_space = vec![false; m * n];
    //     fill_rotate_2(&mut test_space, &test_shape, m, n, x, y);
    //     print_fill(&test_space, m, n);

    //     println!("-------------");
    //     let mut test_space = vec![false; m * n];
    //     fill_rotate_3(&mut test_space, &test_shape, m, n, x, y);
    //     print_fill(&test_space, m, n);

    //     println!("-------------");

    match (shapes, fills) {
        (Ok(s), Ok(mut f)) => {
            // println!("data {d:?}");
            // println!("fills {f:?}");
            let start = Instant::now();
            let result = alpha_fill_shapes(&mut f, &s);
            let time = start.elapsed();
            println!("Gamma version: {} in {:?}", result, time);
        }
        _ => {
            println!("Error in parsing");
        }
    }
}
