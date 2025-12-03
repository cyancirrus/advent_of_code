use std::{error::Error, fs};
// starts at 50
// L**, R**
// password is how many times it hits 0 exactly through any sequence of rotations

// parse data, loop through if it hits zero add one to secret code
// L -> negative nums
// R -> positive nums

pub fn parser(path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let contents = match fs::read_to_string(path) {
        Ok(p) => p,
        Err(e) => return Err(format!("Invalid path {}", e).into()),
    };
    let mut nums = Vec::new();
    for line in contents.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (letter, rest) = line.split_at(1);
        let mut value: i32 = match rest.parse() {
            Ok(v) => v,
            Err(e) => return Err(format!("Invalid number {}", e).into()),
        };
        match letter {
            "L" => value = -value,
            "R" => (),
            _ => return Err(format!("Unknown Prefix {}", letter).into()),
        };
        nums.push(value);
    }
    Ok(nums)
}

pub fn secret_decoder_alpha(nums: &[i32]) -> i32 {
    let mut password: i32 = 0;
    let mut state: i32 = 50;
    let wheel = 100;

    for &n in nums {
        state = (state + wheel + n) % wheel;
        if state == 0 {
            password += 1;
        }
    }
    password
}

pub fn secret_decoder_beta(nums: &[i32]) -> i32 {
    let mut password: i32 = 0;
    let mut state: i32 = 50;
    let wheel = 100;

    for &n in nums {
        let zero_ticks = n.abs() / wheel;
        let n_eff = n % wheel;
        let new_state = (state + wheel + n_eff) % wheel;
        password += zero_ticks;
        if state != 0 {
            if new_state == 0 {
                password += 1;
            } else if n_eff > 0 && new_state < state {
                password += 1;
            } else if n_eff < 0 && new_state > state {
                password += 1;
            }
        }
        state = new_state;
    }
    password
}

// fn main() {
//     let test_nums = vec![-100];
//     assert_eq!(&1, &secret_decoder_beta(&test_nums));

//     let test_nums = vec![-20, -30];
//     assert_eq!(&1, &secret_decoder_beta(&test_nums));

//     let test_nums = vec![-30, 80];
//     assert_eq!(&1, &secret_decoder_beta(&test_nums));

//     let test_nums = vec![-30, 280];
//     assert_eq!(&3, &secret_decoder_beta(&test_nums));

//     let test_nums = vec![-50, 0, 0];
//     assert_eq!(&1, &secret_decoder_beta(&test_nums));

//     let num_parse = parser("./data/day_1.txt");
//     match num_parse {
//         Ok(nums) => {
//             println!("Alpha secret is {}", secret_decoder_alpha(&nums));
//             println!("Beta secret is {}", secret_decoder_beta(&nums));
//         }
//         Err(e) => {
//             println!("unsuccessful parse {}", e);
//         }
//     }
// }
