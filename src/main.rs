#![allow(dead_code, unused)]
use std::{error::Error, fs};

struct Primes {
    primes: Vec<usize>,
    max_checked: usize,
}

impl Primes {
    pub fn new(n: usize) -> Primes {
        let mut primes = vec![2];
        Primes::extender(3, n, &mut primes);
        Primes {
            primes,
            max_checked: n,
        }
    }

    pub fn ensure(&mut self, n: usize) {
        if n < self.max_checked {
            return;
        }
        Self::extender(self.max_checked, n, &mut self.primes);

        self.max_checked = n;
    }

    fn extender(begin: usize, end: usize, primes: &mut Vec<usize>) {
        for n in (begin..=end).step_by(2) {
            let mut is_prime = true;
            for i in 0..primes.len() {
                if n % primes[i] == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                primes.push(n);
            }
        }
    }
}

pub fn parser(path: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
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
            (Ok(l), Ok(r)) => codes.push((l, r)),
            _ => return Err(format!("Invalid Code into integer").into()),
        }
    }
    Ok(codes)
}

fn validate_codes_alpha(codes: &[(usize, usize)]) -> usize {
    let mut sum_invalid_codes = 0;
    for &(left, right) in codes {
        for num in left..=right {
            debug_assert!(
                usize::MAX - num > sum_invalid_codes,
                "overflow increase integer precision"
            );
            let n_str = num.to_string();
            let n_len = n_str.len();
            if n_len & 1 == 0 {
                if &n_str[0..n_len / 2] == &n_str[n_len / 2..] {
                    sum_invalid_codes += num;
                }
            }
        }
    }
    sum_invalid_codes
}

fn validate_codes_beta(codes: &[(usize, usize)], primes: &mut Primes) -> usize {
    let mut sum_invalid_codes = 0;
    for &(left, right) in codes {
        primes.ensure(right.to_string().len() as usize);
        for num in left..=right {
            debug_assert!(
                usize::MAX - num > sum_invalid_codes,
                "overflow increase integer precision"
            );
            let n_str = num.to_string();
            let n_len = n_str.len();
            let mut i = 0;
            let mut is_repeated = true;
            while n_len / primes.primes[i] > 0 {
                let pattern_len = n_len / primes.primes[i];
                i += 1;
                if n_len % pattern_len != 0 {
                    continue;
                }
                let mut pattern = &n_str[0..pattern_len];
                let mut all_match = true;
                for chunk_start in (pattern_len..n_len).step_by(pattern_len) {
                    if &n_str[chunk_start..chunk_start + pattern_len] != pattern {
                        all_match = false;
                        break;
                    }
                }
                if all_match {
                    sum_invalid_codes += num;
                    break;
                }
            }
        }
    }
    sum_invalid_codes
}

fn main() {
    let code_parse = parser("./data/day_2.txt");
    let mut primes = Primes::new(13);
    println!("primes {:?}", primes.primes);
    match code_parse {
        Ok(codes) => {
            println!("Alpha Validate Codes {}", validate_codes_alpha(&codes));
            println!(
                "Beta Validate Codes {}",
                validate_codes_beta(&codes, &mut primes)
            );
        }
        Err(e) => {
            println!("unsuccessful parse {}", e);
        }
    }
}
