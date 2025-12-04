#![allow(dead_code, unused)]
use std::time::Instant;
use std::collections::BTreeSet;
use std::{error::Error, fs};
use std::mem;
use advent_of_code::solutions::day_four::{parser, alpha_neighbor_parse, beta_neighbor_parse, gamma_neighbor_parse};


fn main() {
    let num_parse = parser("./data/day_4.txt");
    match num_parse {
        Ok(grid) => {
            // Single runs
            let a_grid = grid.clone();
            let start = Instant::now();
            let alpha_result = alpha_neighbor_parse(&a_grid);
            let alpha_time = start.elapsed();
            println!("Alpha version: {} in {:?}", alpha_result, alpha_time);
            
            let mut b_grid = grid.clone();
            let start = Instant::now();
            let beta_result = beta_neighbor_parse(&mut b_grid);
            let beta_time = start.elapsed();
            println!("Beta version:  {} in {:?}", beta_result, beta_time);
            
            let mut c_grid = grid.clone();
            let start = Instant::now();
            let gamma_result = gamma_neighbor_parse(&mut c_grid);
            let gamma_time = start.elapsed();
            println!("Gamma version: {} in {:?}", gamma_result, gamma_time);
            
            // Multiple runs for better accuracy
            println!("\n--- Running 100 iterations ---");
            let iterations = 100;
            
            let mut total_alpha = 0u128;
            let mut total_beta = 0u128;
            let mut total_gamma = 0u128;
            
            for _ in 0..iterations {
                let a_grid = grid.clone();
                let start = Instant::now();
                alpha_neighbor_parse(&a_grid);
                total_alpha += start.elapsed().as_nanos();
                
                let mut b_grid = grid.clone();
                let start = Instant::now();
                beta_neighbor_parse(&mut b_grid);
                total_beta += start.elapsed().as_nanos();
                
                let mut c_grid = grid.clone();
                let start = Instant::now();
                gamma_neighbor_parse(&mut c_grid);
                total_gamma += start.elapsed().as_nanos();
            }
            
            let avg_alpha = total_alpha / iterations;
            let avg_beta = total_beta / iterations;
            let avg_gamma = total_gamma / iterations;
            
            println!("Alpha avg: {:?}", std::time::Duration::from_nanos(avg_alpha as u64));
            println!("Beta avg:  {:?}", std::time::Duration::from_nanos(avg_beta as u64));
            println!("Gamma avg: {:?}", std::time::Duration::from_nanos(avg_gamma as u64));
            
            println!("\n--- Relative Performance ---");
            println!("Beta vs Alpha:  {:.2}x", avg_beta as f64 / avg_alpha as f64);
            println!("Gamma vs Beta:  {:.2}x", avg_gamma as f64 / avg_beta as f64);
            println!("Gamma vs Alpha: {:.2}x", avg_gamma as f64 / avg_alpha as f64);
        },
        Err(e) => {
            println!("Unsuccessful error {:?}", e);
        }
    }
}
