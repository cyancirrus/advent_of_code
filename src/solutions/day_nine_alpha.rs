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
fn gamma_find_max_rectangle(points: &mut [(usize, usize)]) -> usize {
    // finds the maximal rectangle for rectangle star convex shapes which thought was the question
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
    for i in 0..points.len() {
        points[i].0 -= min_x;
        points[i].1 -= min_y;
    }

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    // Now use width/height for array sizes
    let mut x_max_y = vec![0; width];
    let mut y_max_x = vec![0; height];
    let mut y_min_x = vec![usize::MAX; height];
    let mut x_min_y = vec![usize::MAX; width];

    for &(p_x, p_y) in points.iter() {
        // find shape boundaries
        x_max_y[p_x] = x_max_y[p_x].max(p_y);
        y_max_x[p_y] = y_max_x[p_y].max(p_x);
        y_min_x[p_y] = y_min_x[p_y].min(p_x);
        x_min_y[p_x] = x_min_y[p_x].min(p_y);
    }
    let mut s_idx = usize::MAX;
    let mut left_scan = vec![0; width.max(height)];
    let mut right_scan = vec![0; width.max(height)];
    // interpolate points by left and right scanning boundaries
    (left_scan[0], right_scan[width - 1]) = (x_max_y[0], x_max_y[width - 1]);
    for idx in 1..width {
        left_scan[idx] = x_max_y[idx].max(left_scan[idx - 1]);
        right_scan[width - idx - 1] = x_max_y[width - idx - 1].max(right_scan[width - idx]);
    }
    for idx in 1..width - 1 {
        x_max_y[idx] = left_scan[idx].min(right_scan[idx]);
    }
    left_scan.fill(0);
    right_scan.fill(0);
    (left_scan[0], right_scan[height - 1]) = (y_max_x[0], y_max_x[height - 1]);
    for idx in 1..height {
        left_scan[idx] = y_max_x[idx].max(left_scan[idx - 1]);
        right_scan[height - idx - 1] = y_max_x[height - idx - 1].max(right_scan[height - idx]);
    }
    for idx in 1..height - 1 {
        y_max_x[idx] = left_scan[idx].min(right_scan[idx]);
    }
    left_scan.fill(usize::MAX);
    right_scan.fill(usize::MAX);
    (left_scan[0], right_scan[height - 1]) = (y_min_x[0], y_min_x[height - 1]);
    println!("y_min_x {y_min_x:?}");
    for idx in 1..height {
        left_scan[idx] = y_min_x[idx].min(left_scan[idx - 1]);
        right_scan[height - idx - 1] = y_min_x[height - idx - 1].min(right_scan[height - idx]);
    }
    for idx in 1..height - 1 {
        y_min_x[idx] = left_scan[idx].max(right_scan[idx]);
    }
    println!("left_scan {left_scan:?}");
    println!("right_scan {right_scan:?}");
    println!("y_min_x {y_min_x:?}");
    left_scan.fill(usize::MAX);
    right_scan.fill(usize::MAX);
    (left_scan[0], right_scan[width - 1]) = (x_min_y[0], x_min_y[width - 1]);
    for idx in 1..width {
        left_scan[idx] = x_min_y[idx].min(left_scan[idx - 1]);
        right_scan[width - idx - 1] = x_min_y[width - idx - 1].min(right_scan[width - idx]);
    }
    for idx in 1..width - 1 {
        x_min_y[idx] = left_scan[idx].max(right_scan[idx]);
    }
    for i in 0..points.len() {
        let (mut x1, mut y1) = points[i];
        for j in i + 1..points.len() {
            let (mut x2, mut y2) = points[j];
            if y1 > y2 {
                mem::swap(&mut x1, &mut x2);
                mem::swap(&mut y1, &mut y2);
            }
            // top left bottom right given
            if (x2 > x1 && y2 > y1
                // inferred top right point
                && x_min_y[x2] <= y1
                && y_max_x[y1] >= x2
                // inferred bottom left point
                && x_max_y[x1] >= y1
                && y_min_x[y2] <= x1)
            {
                max_rectangle = max_rectangle.max((1 + x2 - x1) * (1 + y2 - y1));
            }
            // bottom left top right given
            else if (x1 > x2 && y2 > y1
                // inferred top left point
                && x_min_y[x2] <= y1
                && y_min_x[y1] <= x2
                // inferred bottom right point
                && y_max_x[y2] >= x1
                && x_max_y[x1] >= y2)
            {
                max_rectangle = max_rectangle.max((1 + x1 - x2) * (1 + y2 - y1));
            }
        }
    }
    max_rectangle
}
// // 653744100 ; too low
// // 4601675372 ; too high

// fn main() {
//     let points = parser("./data/day_9.txt");

//     match points {
//         Ok(mut p) => {
//             // let result = alpha_find_max_rectangle( &p);
//             let result = gamma_find_max_rectangle( &mut p);
//             println!("Max found at {result:?}");
//         }
//         _ => {
//             println!("Error in parsing");
//         }
//     }
// }
