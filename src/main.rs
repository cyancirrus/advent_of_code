use std::{fs, error::Error};

pub fn parser(path:&str) -> Result<Vec<(u32, u32)>, Box<dyn Error>>  {
    let contents = match fs::read_to_string(path) {
        Ok(p) => p,
        Err(e) => return Err(format!("Invalid Path {}", e).into()),
    };
    let mut codes:Vec<(u32, u32)> = Vec::new();
    for line in contents.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let identifiers:Option<(&str, &str)> = line.split_once("-");
        match identifiers {
            Some((left, right)) => {
                match (left.parse(), right.parse()) {
                    (Ok(l), Ok(r)) => codes.push((l,r)),
                    _ => return Err(format!("Invalid Code into integer").into())

                }
            },
            _ => return Err(format!("Invalid Split for line code").into()),

        }
    }
    Ok(codes)
}

fn main() {
    let code_parse = parser("./data/day_2.txt");
    match code_parse {
        Ok(codes) => {
            println!("codes \n {:?}", &codes);
        }
        Err(e) => {
            println!("unsuccessful parse {}", e);
        }
    }
}
