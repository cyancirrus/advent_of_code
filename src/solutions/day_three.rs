use std::{error::Error, fs};
const BANK:usize = 100;
const BASE:usize = 12;

pub fn parser(path:&str) -> Result<Vec<[u8;BANK]>, Box<dyn Error>> {
    let mut nums = vec![];
    let batteries = BANK;
    let contents = match fs::read_to_string(path) {
        Ok(p) => p,
        Err(e) => return Err(format!("Invalid Path {}", e).into()),
    };
    for (line_no, line) in contents.lines().enumerate() {
        let mut volts:[u8;BANK] = [0;BANK];
        if line.len() > batteries {
            return Err(format!("Line not BANK chars {}", line_no + 1).into())
        }
        for (idx, ch) in line.chars().enumerate() {
            if let Some(d) = ch.to_digit(10) {
                volts[idx] = d as u8;
            } else {
                return Err(format!("Invalid base 10 voltage").into())
            }
        }
        nums.push(volts);
    }
    Ok(nums)
}

pub fn alpha_max_voltage(racks:&[[u8;BANK]]) -> u64 {
    let mut secret = 0;
    for r in racks {
        let mut max_volt = 0;
        let mut d1 = r[0];
        for &n in &r[1..] {
            let cur = d1 * 10 + n;
            if cur > max_volt {
                max_volt = cur;
            }
            if n > d1 {
                d1 = n;
            }
        }
        secret += max_volt as u64
    }
    secret
}

pub fn beta_max_voltage(racks:&[[u8;BANK]]) -> u64 {
    let mut secret = 0;
    for r in racks {
        let mut positions:[usize; BASE] = std::array::from_fn(|i| BANK - 1 - i);
        let mut voltages:[u8; BASE] = [0;BASE];
        for i in (0..BASE).rev() {
            let start = if i == BASE - 1 { 0 } else { positions[i+1] + 1 };
            let end = positions[i];
            for j in start..=end {
                if r[j] > voltages[i] {
                    positions[i] = j;
                    voltages[i] = r[j];
                }
            }
        }
        let mut total_volts = 0;
        let mut b_ten = 1;
        for v in voltages {
            total_volts += v as u64 * b_ten;
            b_ten *= 10;

        }
        secret += total_volts;
    }
    secret
}

// fn main() {
//     let mut num_parse = parser("./data/day_3.txt");
//     match num_parse {
//         Ok(bats) => {
//             println!("Alpha version {}", alpha_max_voltage(&bats));
//             println!("Beta version {}", beta_max_voltage(&bats));
//         },
//         Err(e) => {
//             println!("Unsuccessful error {:?}", e);
//         }
//     }
    
// }
