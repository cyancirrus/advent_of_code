use std::{fs, error::Error};

pub fn parser(path:&str) -> Result<Vec<(u64, u64)>, Box<dyn Error>>  {
    let contents = match fs::read_to_string(path) {
        Ok(p) => p,
        Err(e) => return Err(format!("Invalid Path {}", e).into()),
    };
    let mut codes = Vec::new();
    for line in contents.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (left, right) = line.split_once("-").ok_or("Invalid split for line code")?;
        match (left.parse(), right.parse()) {
            (Ok(l), Ok(r)) => codes.push((l,r)),
            _ => return Err(format!("Invalid Code into integer").into())
        }
    }
    Ok(codes)
}

fn validate_codes_alpha(codes:Vec<(u64, u64)>) -> u64 {
    let mut sum_invalid_codes = 0;
    for (left, right) in codes {
        for num in left..=right {
            debug_assert!(u64::MAX - num > sum_invalid_codes, "overflow increase integer precision");
            let n_str = num.to_string();
            let n_len = n_str.len();
            if n_len & 1 == 0 {
                if &n_str[0..n_len/2] == &n_str[n_len/2..] { sum_invalid_codes += num; }
            }
        }
    }
    sum_invalid_codes
}



fn main() {
    let code_parse = parser("./data/day_2.txt");
    match code_parse {
        Ok(codes) => {
            println!("Validate codes {}", validate_codes_alpha(codes));
        }
        Err(e) => {
            println!("unsuccessful parse {}", e);
        }
    }
}
