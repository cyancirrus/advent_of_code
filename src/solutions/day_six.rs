#![allow(dead_code, unused)]
use std::mem;
use std::{error::Error, fs};

const INPUT_CARD: usize = 4;

#[derive(Debug)]
pub enum Opperation {
    Product,
    Summand,
}
#[derive(Debug)]
pub struct Problem {
    op: Opperation,
    nums: Vec<u64>,
}

pub fn parser_alpha(path: &str) -> Result<Vec<Problem>, Box<dyn Error>> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(("Path: {path:?} does not exist").into()),
    };
    let mut problems = vec![];
    for (j, line) in content.lines().rev().enumerate() {
        if j > INPUT_CARD {
            return Err(("Unexpected number of entries").into());
        };
        let cols: Vec<&str> = line.split_whitespace().collect();
        for (i, s) in cols.iter().enumerate() {
            if j == 0 {
                let op = match *s {
                    "*" => Opperation::Product,
                    "+" => Opperation::Summand,
                    _ => return Err(("Unexpected symbol for opperation").into()),
                };
                problems.push(Problem {
                    op,
                    nums: vec![0; INPUT_CARD],
                });
            } else {
                match s.parse() {
                    Ok(v) => problems[i].nums[INPUT_CARD - j] = v,
                    Err(e) => return Err(("Parsing failed for value ({i}, {j})").into()),
                }
            }
        }
    }
    Ok(problems)
}

fn parser_beta(path: &str) -> Result<Vec<Problem>, Box<dyn Error>> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(("Path: {path:?} does not exist").into()),
    };
    let mut problems = vec![];
    let mut lines: Vec<&str> = content.lines().collect();
    let mut digits = vec![];
    if let Some(line) = lines.pop() {
        let mut length = 0;
        for &b in line.as_bytes() {
            let op = match b {
                b'*' => Opperation::Product,
                b'+' => Opperation::Summand,
                b' ' => {
                    length += 1;
                    continue;
                },
                _ => return Err(("Unexpected symbol found").into()),
            };
            problems.push(Problem { op, nums: vec![] } );
            digits.push(length);
            length = 0;
        }
            // Opperations start with index 0 -> everything shifted by 1
        digits.push(length + 1);
    };
    for line in lines {
        // problem index
        let mut p_idx = 0;
        // digit index
        let mut d_idx = 0;
        for b in line.bytes() {
            // Opperations start with index 0 -> everything shifted by 1
            if d_idx == digits[p_idx+1] {
                d_idx = 0;
                p_idx += 1;
                continue;
            }
            if d_idx >= problems[p_idx].nums.len() {
                problems[p_idx].nums.push(0);
            }
            if b != b' ' { problems[p_idx].nums[d_idx] = problems[p_idx].nums[d_idx] * 10 + (b - b'0') as u64; }
            d_idx += 1;
        }
    }
    Ok(problems)
}
pub fn evaluate_squid_math(problems: &[Problem]) -> u64 {
    let mut result = 0;
    for p in problems {
        match p.op {
            Opperation::Product => {
                // identity
                let mut prod = 1;
                for n in &p.nums {
                    prod *= n;
                }
                result += prod;
            }
            Opperation::Summand => {
                //identity
                let mut sum = 0;
                for n in &p.nums {
                    sum += n;
                }
                result += sum;
            }
        }
    }
    result
}

// fn main() {
//     let probs = parser_alpha("./data/day_6.txt");
//     match probs {
//         Ok(p) => {
//             println!("Alpha squid result {}", evaluate_squid_math(&p));
//         }
//         _ => {
//             println!("Error in parsing");
//         }
//     }
//     let probs = parser_beta("./data/day_6.txt");
//     match probs {
//         Ok(p) => {
//             println!("Beta squid result {}", evaluate_squid_math(&p));
//         }
//         _ => {
//             println!("Error in parsing");
//         }
//     }
// }
