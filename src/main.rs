#![allow(dead_code, unused)]
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem;
use std::time::Instant;
use std::{error::Error, fs};

const NUMBER_SHAPES: usize = 6;
const N: usize = 3;

fn parser_shapes(path: &str) -> Result<Vec<Vec<bool>>, Box<dyn Error>> {
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

fn parser_fills(path: &str) -> Result<Vec<(usize, usize, Vec<usize>)>, Box<dyn Error>> {
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

fn fill_rotate_0(
    shape: &[bool],
    space: &mut [bool],
    m: usize,
    n: usize,
    x: usize,
    y: usize,
) -> bool {
    if x + N > m || y + N > n {
        return false;
    }
    for i in 0..N {
        for j in 0..N {
            if shape[i * N + j] && space[(i + x) * m + j + y] {
                return false;
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            space[(i + x) * m + j + y] = shape[i * N + j];
        }
    }
    true
}

fn fill_rotate_1(
    shape: &[bool],
    space: &mut [bool],
    m: usize,
    n: usize,
    x: usize,
    y: usize,
) -> bool {
    // j decrease i increase
    if x + N > m || y + N > n {
        return false;
    }
    for i in 0..N {
        for j in 0..N {
            if shape[i * N + j] && space[(j +  x) * m + N - i - 1 + y] {
                return false;
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            space[(j + x) * m + N - i - 1 + y] = shape[i * N + j];
        }
    }
    true
}

fn fill_rotate_2(
    shape: &[bool],
    space: &mut [bool],
    m: usize,
    n: usize,
    x: usize,
    y: usize,
) -> bool {
    if x + N > m || y + N > n {
        return false;
    }
    for i in 0..N {
        for j in 0..N {
            if shape[i * N + j] && space[(N - i  - 1 + x) * m + N - j  - 1 + y] {
                return false;
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            space[(N - i  - 1 + x) * m + N - j  - 1 + y] = shape[i * N + j];
        }
    }
    true
}

fn fill_rotate_3(
    shape: &[bool],
    space: &mut [bool],
    m: usize,
    n: usize,
    x: usize,
    y: usize,
) -> bool {
    // j decrease i increase
    if x + N > m || y + N > n {
        return false;
    }
    for i in 0..N {
        for j in 0..N {
            if shape[i * N + j] && space[( N -j +  x - 1) * m + i + y] {
                return false;
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            space[( N -j +  x - 1) * m + i + y] = shape[i * N + j];
        }
    }
    true
}

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

// fn fill_shape(shapes: &[Vec<bool>], (m, n, idx_count): (usize, usize, Vec<usize>)) -> bool {
//     let mut space = vec![false; m * n];

//     false
// }

fn main() {
    let shapes = "./data/day_12_sample_shapes.txt";
    let fills = "./data/day_12_sample_fills.txt";
    let data = parser_shapes(shapes);
    let fills = parser_fills(fills);
    let test_shape = vec![
        true, true, true,
        true, false, false,
        true, true, true
    ];
    let (m, n) = (4, 4);
    let (x, y) = (1, 1);
    print_fill(&test_shape, 3, 3);
    
    println!("-------------"); 
    let mut test_space = vec![false; m * n];
    fill_rotate_0(&test_shape, &mut test_space, m, n, x, y);
    print_fill(&test_space, m, n);
    
    println!("-------------"); 
    let mut test_space = vec![false; m * n];
    fill_rotate_1(&test_shape, &mut test_space, m, n, x, y);
    print_fill(&test_space, m, n);
    
    println!("-------------"); 
    let mut test_space = vec![false; m * n];
    fill_rotate_2(&test_shape, &mut test_space, m, n, x, y);
    print_fill(&test_space, m, n);


    println!("-------------"); 
    let mut test_space = vec![false; m * n];
    fill_rotate_3(&test_shape, &mut test_space, m, n, x, y);
    print_fill(&test_space, m, n);
    
    println!("-------------"); 

    // match (data, fills) {
    //     (Ok(d), Ok(f)) => {
    //         println!("data {d:?}");
    //         println!("fills {f:?}");
    //         // let start = Instant::now();
    //         // let result = gamma_find_number_paths(&c);
    //         // let time = start.elapsed();
    //         // println!("Gamma version: {} in {:?}", result, time);
    //         // let start = Instant::now();
    //         // let result = beta_find_number_paths(&mut c);
    //         // let time = start.elapsed();
    //         // println!("Beta version: {:?} in {:?}", result, time);
    //         // let start = Instant::now();
    //         // let result = beta_find_number_paths(&mut c);
    //         // let time = start.elapsed();
    //         // println!("Beta version: {} in {:?}", result, time);
    //     }
    //     _ => {
    //         println!("Error in parsing");
    //     }
    // }
}
