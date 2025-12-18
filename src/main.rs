#![allow(dead_code, unused)]
use std::collections::HashMap;
use std::mem;
use std::time::Instant;
use std::{error::Error, fs};

// area = (xmax - xmin) * (ymax - ymin);
// d area / dxmax = (ymax - ymin); // this feels nonlinear but looks linear
// d area / dxmin = - (ymax - ymin);

// d area / dymax = (xmax - xmin);
// d area / dymin = - (xmax - xmin);

// lets try to analyze in terms of the total differential
// d area / d max = (ymax-ymin) dxmax +  (xmax - xmin) dymax // that's the nonlinear part

// hmm need like a frontier

const EPSILON: f64 = 1e-6;

fn parser(path: &str) -> Result<Vec<(isize, isize)>, Box<dyn Error>> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(format!("Path not found {path:?}").into()),
    };
    let mut points: Vec<(isize, isize)> = vec![];
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

fn are_clockwise_edges_valid(
    e1p1: &(isize, isize),
    e1p2: &(isize, isize), e2p0: &(isize, isize),
    e2p1: &(isize, isize),
    e2p2: &(isize, isize),
    e2p3: &(isize, isize),
) -> bool {
    let e1 = (e1p2.0 - e1p1.0, e1p2.1 - e1p1.1);
    let e2 = (e2p2.0 - e2p1.0, e2p2.1 - e2p1.1);
    
    let e1_len_sq = e1.0 * e1.0 + e1.1 * e1.1;
    let e2_len_sq = e2.0 * e2.0 + e2.1 * e2.1;
    
    // Handle zero-length edges
    if e1_len_sq == 0 || e2_len_sq == 0 {
        return true;
    }

    let d = (e2p1.0 - e1p1.0, e2p1.1 - e1p1.1);
    let denom = e2.0 * e1.1 - e1.0 * e2.1;
    
    // Parallel edges
    if denom.abs() == 0 {
        let mut valid = true;
        let cross = d.0 * e1.1 - d.1 * e1.0;
    
        // Collinear edges
        if cross.abs() == 0 {
            let dot = e2.0 * e1.0 + e2.1 * e1.1;
            
        let use_x = e1.0.abs() > e1.1.abs();
        
        // Use actual traversal positions, not min/max
        let (rect_start, rect_end, loop_start, loop_end) = if use_x {
            (e1p1.0, e1p2.0, e2p1.0, e2p2.0)
        } else {
            (e1p1.1, e1p2.1, e2p1.1, e2p2.1)
        };
        
        // Check for overlap in bounding boxes
        let (rect_min, rect_max) = if rect_start < rect_end {
            (rect_start, rect_end)
        } else {
            (rect_end, rect_start)
        };
        let (loop_min, loop_max) = if loop_start < loop_end {
            (loop_start, loop_end)
        } else {
            (loop_end, loop_start)
        };
        
        // If they only touch at one point (no proper overlap)
        if rect_max == loop_min || rect_min == loop_max {
            return true;  // Just touching at endpoints
        }
        
        // No overlap at all
        if rect_max < loop_min || loop_max < rect_min {
            return true;
        }
        
        // They overlap - must point in same direction
        if dot <= 0 {
            return false;
        }
        
        // ... rest of extension checks ...
            
            let use_x = e1.0.abs() > e1.1.abs();
            
            // Use actual traversal positions, not min/max
            let (rect_start, rect_end, loop_start, loop_end) = if use_x {
                (e1p1.0, e1p2.0, e2p1.0, e2p2.0)
            } else {
                (e1p1.1, e1p2.1, e2p1.1, e2p2.1)
            };
            
            // Check if rectangle extends before polygon edge starts (in traversal direction)
            // This means we need to check the turn INTO the polygon edge
            if (rect_start < loop_start && e1.0 + e1.1 > 0) || 
               (rect_start > loop_start && e1.0 + e1.1 < 0) {
                let prev_turn = (e2p1.0 - e2p0.0) * e2.1 - (e2p1.1 - e2p0.1) * e2.0;
                valid &= prev_turn <= 0;
            }
            
            // Check if rectangle extends after polygon edge ends (in traversal direction)
            // This means we need to check the turn OUT OF the polygon edge
            if (rect_end > loop_end && e1.0 + e1.1 > 0) || 
               (rect_end < loop_end && e1.0 + e1.1 < 0) {
                let next_turn = e2.0 * (e2p3.1 - e2p2.1) - e2.1 * (e2p3.0 - e2p2.0);
                valid &= next_turn <= 0;
            }
            
            // No overlap check - if they don't overlap at all, return early
            let (rect_min, rect_max) = if rect_start < rect_end {
                (rect_start, rect_end)
            } else {
                (rect_end, rect_start)
            };
            let (loop_min, loop_max) = if loop_start < loop_end {
                (loop_start, loop_end)
            } else {
                (loop_end, loop_start)
            };
            
            if rect_max < loop_min || loop_max < rect_min {
                return true;
            }
        }
        // Parallel but not collinear
        return valid;
    }
    
    // Non-parallel edges - check for intersection
    let s = (d.1 * e2.0 - d.0 * e2.1);
    let t = (e1.0 * d.1 - d.0 * e1.1);
    let (s, t, denom) = if denom > 0 {
        (s, t, denom)
    } else {
        (-s, -t, -denom)
    };
    
    // Valid if not a proper interior intersection
    !(0 < s && s < denom && 0 < t && t < denom)
}

fn delta_find_max_rectangle(points: &mut [(isize, isize)]) -> isize {
    let n = points.len();
    let mut max_rectangle = 0;
    // for jdx in 4..=4 {
    //     for kdx in 6..=6 {
    for jdx in 0..n {
        for kdx in jdx + 1..n {
            let mut valid = true;
            // top-left, bottom-right, top-right, bottom-left
            let tl = (
                points[jdx].0.min(points[kdx].0),
                points[jdx].1.min(points[kdx].1),
            );
            let br = (
                points[jdx].0.max(points[kdx].0),
                points[jdx].1.max(points[kdx].1),
            );
            // println!("Point {tl:?}, {br:?}");
            let tr = (br.0, tl.1);
            let bl = (tl.0, br.1);
            let potential = (1 + tr.0 - tl.0) * (1 + bl.1 - tl.1);
            if potential <= max_rectangle {
                continue;
            }
            let mut valid = true;
            for idx in 0..n {
                // clockwise edges
                let (x0, x1, x2, x3) = (points[idx], points[(idx + 1) % n], points[(idx + 2) % n], points[(idx + 3)%n]);
                let v1 = are_clockwise_edges_valid(&tl, &tr, &x0, &x1, &x2, &x3);
                let v2 = are_clockwise_edges_valid(&tr, &br, &x0, &x1, &x2, &x3);
                let v3 = are_clockwise_edges_valid(&br, &bl, &x0, &x1, &x2, &x3);
                let v4 = are_clockwise_edges_valid(&bl, &tl, &x0, &x1, &x2, &x3);
                if !v1 || !v2 || !v3 || !v4 {
                    // println!("Failure for edge testing ({x1:?}, {x2:?}");
                    // println!("TL->TR {}, TR->BR {}, BR-> BL {}, BL->TL {}", v1, v2, v3, v4);

                    valid = false;
                    break;
                }
            }
            // for idx in 0..n {
            //     // clockwise edges
            //     let (x0, x1, x2, x3) = (points[idx], points[(idx + 1) % n], points[(idx + 2) % n], points[(idx + 3)%n]);
            //     if !are_clockwise_edges_valid(&tl, &tr, &x0, &x1, &x2, &x3)
            //         || !are_clockwise_edges_valid(&tr, &br, &x0, &x1, &x2, &x3)
            //         || !are_clockwise_edges_valid(&br, &bl, &x0, &x1, &x2, &x3)
            //         || !are_clockwise_edges_valid(&bl, &tl, &x0, &x1, &x2, &x3)
            //     {
            //         valid = false;
            //         break;
            //     }
            // }
            if valid {
                max_rectangle = potential;
            }
        }
    }
    max_rectangle
}

fn main() {
    // NOTE: Approach is strictly dominated for larger inputs
    // println!("-------------------------------------------------------------");
    // let points = parser("./data/day_9.txt");
    // match points {
    //     Ok(mut p) => {
    //         // let result = alpha_find_max_rectangle(&p);
    //         // println!("Alpha Max found at {result:?}");
    //         let start = Instant::now();
    //         let result = beta_find_max_rectangle(&mut p);
    //         let time = start.elapsed();
    //         println!("Beta version: {} in {:?}", result, time);
    //     }
    //     _ => {
    //         println!("Error in parsing");
    //     }
    // }
    println!("-------------------------------------------------------------");
    let points = parser("./data/day_9.txt");
    match points {
        Ok(mut p) => {
            let start = Instant::now();
            let result = delta_find_max_rectangle(&mut p);
            let time = start.elapsed();
            println!("Delta version: {} in {:?}", result, time);
        }
        _ => {
            println!("Error in parsing");
        }
    }
}
