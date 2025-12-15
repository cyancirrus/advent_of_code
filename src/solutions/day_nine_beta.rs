#![allow(dead_code, unused)]
use std::collections::HashMap;
use std::mem;
use std::{error::Error, fs};

// area = (xmax - xmin) * (ymax - ymin);
// d area / dxmax = (ymax - ymin); // this feels nonlinear but looks linear
// d area / dxmin = - (ymax - ymin);

// d area / dymax = (xmax - xmin);
// d area / dymin = - (xmax - xmin);

// lets try to analyze in terms of the total differential
// d area / d max = (ymax-ymin) dxmax +  (xmax - xmin) dymax // that's the nonlinear part

// hmm need like a frontier

fn parser(path: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(format!("Path not found {path:?}").into()),
    };
    let mut points: Vec<(usize, usize)> = vec![];
    for line in content.lines() {
        let (x_str, y_str) = line.split_once(",").ok_or("Unexpected delimiter found")?;
        match (x_str.parse(), y_str.parse()) {
            (Ok(x_val), Ok(y_val)) => points.push((x_val, y_val)),
            _ => {
                return Err(
                    format!("Unexpected parsing error for values ({x_str:?}, {y_str:?})").into(),
                );
            }
        };
    }
    Ok(points)
}

fn alpha_find_max_rectangle(points: &[(usize, usize)]) -> usize {
    // let mut min_pos = vec![usize::MAX;n];
    // let mut max_pos = vec![0;n];
    let mut min_pos = HashMap::new();
    let mut max_pos = HashMap::new();

    for &(p_x, p_y) in points {
        let v = min_pos.entry(p_x).or_insert(usize::MAX);
        *v = (*v).min(p_y);
        let v = max_pos.entry(p_x).or_insert(0);
        *v = (*v).max(p_y);
    }
    let mut max_rectangle = 0;
    for (min_x, min_y) in min_pos.iter() {
        for (max_x, max_y) in max_pos.iter() {
            if max_x > min_x && max_y > min_y {
                max_rectangle = max_rectangle.max((1 + max_x - min_x) * (1 + max_y - min_y));
            }
            if min_x > max_x && max_y > min_y {
                max_rectangle = max_rectangle.max((1 + min_x - max_x) * (1 + max_y - min_y));
            }
        }
    }
    max_rectangle
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for &cell in row {
            print!("{}", if cell { '@' } else { '.' });
        }
        println!();
    }
}
fn print_grid_transposed(grid: &Vec<Vec<char>>) {
    if grid.is_empty() || grid[0].is_empty() {
        return;
    }
    
    let cols = grid[0].len();
    
    for col in 0..cols {
        for row in grid {
            print!("{}", row[col] );
        }
        println!();
    }
}

fn beta_find_max_rectangle(points: &mut [(usize, usize)]) -> usize {
    let mut max_rectangle = 0;
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_y = 0;
    let mut max_x = 0;
    for &(x, y) in points.iter() {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
        max_x = max_x.max(x);
    }
    if min_x == usize::MAX || min_y == usize::MAX || max_x == 0 || max_y == 0 {
        return 0;
    }
    let (width, height) = (1 + max_x - min_x, 1 + max_y - min_y);
    println!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y);
    println!("width: {}, height: {}", width, height);
    let mut grid = vec![vec![b'.'; height]; width];
    for i in 0..points.len() {
        points[i].0 -= min_x;
        points[i].1 -= min_y;
    }
    println!("parsing grid boundaries");
    for idx in 0..points.len() {
        let start = points[idx];
        let end = points[(idx + 1) % points.len()];
        grid[start.0][start.1] = b'@';
        grid[end.0][end.1] = b'@';
        if start.0 == end.0 {
            if start.1 < end.1 {
                for j in start.1+1..end.1 {
                    grid[start.0][j] = b'*';
                }
            } else {
                for j in end.1+1..start.1 {
                    grid[end.0][j] = b'*';
                }
            }
        } else if start.1 == end.1 {
            if start.0 < end.0 {
                for i in start.0+1..end.0 {
                    grid[i][start.1] = b'*';
                }
            } else {
                for i in end.0+1..start.0 {
                    grid[i][end.1] = b'*';
                }
            }
        }
    }
    println!("parsing grid interior");
    // print_grid_transposed(&grid);
    let neighbors = [!0, 1,];
    for i in 0..width {
        let mut even = true;
        let mut ignore = false;
        // (-1: left, 0: none, 1: right )
        let mut edge = 0;
        for j in 0..height {
            if edge == 0 && grid[i][j] == b'@' {
                for dx in neighbors {
                    let ni = i.wrapping_add(dx);
                    if ni < width && grid[ni][j] == b'*' {
                        if dx == 1 { edge = 1; }
                        else if dx == !0 { edge = -1; }
                    }
                }
                continue;
            } else if edge!= 0 && grid[i][j] == b'@' {
                for dx in neighbors {
                    let ni = i.wrapping_add(dx);
                    if ni < width && grid[ni][j] == b'*' {
                        if dx == 1 && edge == 1 {
                            // exterior edges
                            edge = 0;
                            continue;
                        } else if dx == !0 && edge == -1 {
                            // exterior edges
                            edge = 0;
                            continue;
                        } else {
                            // interior edge
                            even = !even;
                            edge = 0;
                        }
                    }
                }
            } else if edge != 0 {
                continue;
            } else if grid[i][j] == b'*' {
                even = !even;
            } else if !even {
                grid[i][j] = b'*';
            }
        }
    }
    // print_grid_transposed(&grid);
    println!("finding optimal nodes");
    for k in 0..points.len() {
        println!("k {k:?}");
        for l in 0..points.len() {
            let mut cancel = false;
            let (base, aux) = (points[k], points[l]);
            let result = (1 + base.0.max(aux.0) - base.0.min(aux.0)) * (1 + base.1.max(aux.1) - base.1.min(aux.1));
            if base.0 == aux.0 || base.1 == aux.1 || result < max_rectangle {
                continue;
            } else if base.0 < aux.0 && base.1 < aux.1 {
                // top left, bottom right
                for i in base.0..=aux.0 {
                    if grid[i][base.1] == b'.' || grid[i][aux.1] == b'.' {
                        cancel = true;
                        break;
                    }
                }
                if cancel { continue; }
                for j in base.1..=aux.1 {
                    if grid[base.0][j] == b'.' || grid[base.0][j] == b'.' {
                        cancel = true;
                        break;
                    }
                }
                if cancel { continue; }
            } else if base.0 < aux.0 && aux.1 < base.1 {
                // bottom left, top right
                for i in base.0..=aux.0 {
                    if grid[i][base.1] == b'.' || grid[i][aux.1] == b'.' {
                        cancel = true;
                        break;
                    }
                }
                if cancel { continue;}
                for j in aux.1..=base.1 {
                    if grid[base.0][j] == b'.' || grid[base.0][j] == b'.' {
                        cancel = true;
                        break;
                    }
                }
                if cancel {
                    continue;
                }
            } else {
                continue;
            }
            max_rectangle = max_rectangle.max(result);
        }
    }
    max_rectangle
}
// 653744100 ; too low
// 4601675372 ; too high

// fn main() {
//     let points = parser("./data/day_9.txt");
//     match points {
//         Ok(mut p) => {
//             let result = alpha_find_max_rectangle(&p);
//             println!("Alpha Max found at {result:?}");
//             let result = beta_find_max_rectangle(&mut p);
//             println!("Beta Max found at {result:?}");
//         }
//         _ => {
//             println!("Error in parsing");
//         }
//     }
// }
