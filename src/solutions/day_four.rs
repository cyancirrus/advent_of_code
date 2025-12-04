#![allow(dead_code, unused)]
// use std::time::Instant;
use std::collections::BTreeSet;
use std::{error::Error, fs};
use std::mem;

const WIDTH:usize = 137;

pub fn parser(path:&str) -> Result<Vec<Vec<i8>>, Box<dyn Error>> {
    let mut grid = Vec::new();    
    let content = match fs::read_to_string(path) {
        Ok(lines) => lines,
        Err(e) => return Err(format!("File not found {:?}", path).into()),
    };

    for line in content.lines() {
        // adding padding
        let mut encoding:Vec<i8> = vec![0;WIDTH];
        for (i, ch) in line.chars().enumerate() {
            if i > WIDTH {
                return Err(format!("Impropper length found index {i}").into())
            }
            match ch {
                '@' => { encoding[i] = 1; },
                '.' => { encoding[i] = 0; },
                _ => return Err(format!("Symbol not expected returning error").into()),

            }
        }
        grid.push(encoding)
    }
    Ok(grid)
}

pub fn alpha_neighbor_parse(grid:&[Vec<i8>]) -> usize {
    let mut nodes = 0;
    let threshold = 4;
    if grid.is_empty() || grid[0].is_empty() { return 0; }
    let (m,n) = (grid.len() as isize, grid[0].len() as isize);

    for x in 0..m {
        for y in 0..n {
            if grid[x as usize][y as usize] == 0 { continue; }
            // could do like the like overflow sub trick but this is fine just a bit of casts which
            // turn into no ops
            let mut neighbors = 0;
            for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (0,1), (1,1), (1,0), (1,-1), (0,-1)] {
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if 0 <= nx && nx < m && 0 <= ny && ny < n {
                    neighbors += grid[nx as usize ][ny as usize];
                }
            }
            if neighbors < threshold {
                nodes += 1;
            }
        }
    }
    nodes
}

pub fn beta_neighbor_parse(grid:&mut [Vec<i8>]) -> usize {
    let mut nodes = 0;
    let threshold = 4;
    if grid.is_empty() || grid[0].is_empty() { return 0; }
    let (m,n) = (grid.len() as isize, grid[0].len() as isize);
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0,1), (1,1), (1,0), (1,-1), (0,-1)];
    
    let mut stack = vec![];
    for i in 0..m {
        for j in 0..n {
            if grid[i as usize][j as usize] == 1 {
                stack.push((i,j));
            }
        }
    }
    
    // depth first or breadth first will work
    while let Some((x,y)) = stack.pop() {
        // could do like the like overflow sub trick but this is fine just noop casts
        if grid[x as usize][y as usize] == 0 { continue; }
        let mut neighbors = 0;
        for (dx, dy) in directions {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if 0 <= nx && nx < m && 0 <= ny && ny < n  {
                neighbors += grid[nx as usize ][ny as usize];
            }
        }
        if neighbors < threshold {
            grid[x as usize][y as usize] = 0;
            nodes += 1;
            for (dx, dy) in directions {
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if 0 <= nx && nx < m && 0 <= ny && ny < n && grid[nx as usize][ny as usize] == 1 {
                    stack.push((nx, ny));
                }
            }
        }
    }
    nodes
}

pub fn gamma_neighbor_parse(grid:&mut [Vec<i8>]) -> usize {
    let mut nodes = 0;
    let threshold = 4;
    if grid.is_empty() || grid[0].is_empty() { return 0; }
    let (m,n) = (grid.len() as isize, grid[0].len() as isize);
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0,1), (1,1), (1,0), (1,-1), (0,-1)];
    
    let mut stack = BTreeSet::new();
    for i in 0..m {
        for j in 0..n {
            if grid[i as usize][j as usize] == 1 {
                stack.insert((i,j));
            }
        }
    }
    
    // depth first or breadth first will work
    while let Some((x,y)) = stack.pop_first() {
        // could do like the like overflow sub trick but this is fine just noop casts
        if grid[x as usize][y as usize] == 0 { continue; }
        let mut neighbors = 0;
        for (dx, dy) in directions {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if 0 <= nx && nx < m && 0 <= ny && ny < n  {
                neighbors += grid[nx as usize ][ny as usize];
            }
        }
        if neighbors < threshold {
            grid[x as usize][y as usize] = 0;
            nodes += 1;
            for (dx, dy) in directions {
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if 0 <= nx && nx < m && 0 <= ny && ny < n && grid[nx as usize][ny as usize] == 1 {
                    stack.insert((nx, ny));
                }
            }
        }
    }
    nodes
}


// fn main() {
//     let num_parse = parser("./data/day_4.txt");
//     match num_parse {
//         Ok(grid) => {
//             // Single runs
//             let a_grid = grid.clone();
//             let start = Instant::now();
//             let alpha_result = alpha_neighbor_parse(&a_grid);
//             let alpha_time = start.elapsed();
//             println!("Alpha version: {} in {:?}", alpha_result, alpha_time);
            
//             let mut b_grid = grid.clone();
//             let start = Instant::now();
//             let beta_result = beta_neighbor_parse(&mut b_grid);
//             let beta_time = start.elapsed();
//             println!("Beta version:  {} in {:?}", beta_result, beta_time);
            
//             let mut c_grid = grid.clone();
//             let start = Instant::now();
//             let gamma_result = gamma_neighbor_parse(&mut c_grid);
//             let gamma_time = start.elapsed();
//             println!("Gamma version: {} in {:?}", gamma_result, gamma_time);
            
//             // Multiple runs for better accuracy
//             println!("\n--- Running 100 iterations ---");
//             let iterations = 100;
            
//             let mut total_alpha = 0u128;
//             let mut total_beta = 0u128;
//             let mut total_gamma = 0u128;
            
//             for _ in 0..iterations {
//                 let a_grid = grid.clone();
//                 let start = Instant::now();
//                 alpha_neighbor_parse(&a_grid);
//                 total_alpha += start.elapsed().as_nanos();
                
//                 let mut b_grid = grid.clone();
//                 let start = Instant::now();
//                 beta_neighbor_parse(&mut b_grid);
//                 total_beta += start.elapsed().as_nanos();
                
//                 let mut c_grid = grid.clone();
//                 let start = Instant::now();
//                 gamma_neighbor_parse(&mut c_grid);
//                 total_gamma += start.elapsed().as_nanos();
//             }
            
//             let avg_alpha = total_alpha / iterations;
//             let avg_beta = total_beta / iterations;
//             let avg_gamma = total_gamma / iterations;
            
//             println!("Alpha avg: {:?}", std::time::Duration::from_nanos(avg_alpha as u64));
//             println!("Beta avg:  {:?}", std::time::Duration::from_nanos(avg_beta as u64));
//             println!("Gamma avg: {:?}", std::time::Duration::from_nanos(avg_gamma as u64));
            
//             println!("\n--- Relative Performance ---");
//             println!("Beta vs Alpha:  {:.2}x", avg_beta as f64 / avg_alpha as f64);
//             println!("Gamma vs Beta:  {:.2}x", avg_gamma as f64 / avg_beta as f64);
//             println!("Gamma vs Alpha: {:.2}x", avg_gamma as f64 / avg_alpha as f64);
//         },
//         Err(e) => {
//             println!("Unsuccessful error {:?}", e);
//         }
//     }
// }
