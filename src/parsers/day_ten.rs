use std::{error::Error, fs};

fn parse_bit_line(line: &str) -> Result<(u16, Vec<u16>, Vec<u16>), Box<dyn Error>> {
    let mut chars = line.chars().peekable();
    // Parse lights pattern [.##.]
    let mut d = 0u16;
    let mut pos = 1u16;

    let mut switches = Vec::new();
    let mut costs = Vec::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            '[' => {
                chars.next();
                loop {
                    match chars.peek() {
                        Some(&']') => {
                            chars.next();
                            break;
                        }
                        Some(&'.') => {
                            pos <<= 1;
                            chars.next();
                        }
                        Some(&'#') => {
                            d |= pos;
                            pos <<= 1;
                            chars.next();
                        }
                        _ => return Err(format!("Unexpected char in lights: {}", ch).into()),
                    }
                }
            }
            '(' => {
                // consume '('
                chars.next();
                let mut s = 0u16;
                let mut num_str = String::new();
                loop {
                    match chars.peek() {
                        Some(&',') => {
                            chars.next();
                            if !num_str.is_empty() {
                                let n: usize = num_str.parse()?;
                                s |= 1 << n;
                                num_str.clear();
                            }
                        }
                        Some(&')') => {
                            chars.next();
                            if !num_str.is_empty() {
                                let n: usize = num_str.parse()?;
                                s |= 1 << n;
                            }
                            switches.push(s);
                            break;
                        }
                        Some(&c) if c.is_ascii_digit() => {
                            num_str.push(c);
                            chars.next();
                        }
                        _ => return Err("Unexpected char in switch group".into()),
                    }
                }
            }
            '{' => {
                chars.next(); // consume '{'
                let mut num_str = String::new();

                loop {
                    match chars.peek() {
                        Some(&',') => {
                            chars.next();
                            if !num_str.is_empty() {
                                costs.push(num_str.parse()?);
                                num_str.clear();
                            }
                        }
                        Some(&'}') => {
                            chars.next();
                            if !num_str.is_empty() {
                                costs.push(num_str.parse()?);
                            }
                            break;
                        }
                        Some(&c) if c.is_ascii_digit() => {
                            num_str.push(c);
                            chars.next();
                        }
                        _ => return Err("Unexpected char in cost group".into()),
                    }
                }
            }
            ' ' => {
                chars.next();
            }
            _ => return Err(format!("Unexpected char: {}", ch).into()),
        }
    }
    Ok((d, switches, costs))
}

fn parse_int_line(line: &str) -> Result<(Vec<Vec<u16>>, Vec<u16>), Box<dyn Error>> {
    let mut chars = line.chars().peekable();
    // Parse lights pattern [.##.]

    let mut switches = Vec::new();
    let mut costs = Vec::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            '[' => {
                chars.next();
                loop {
                    match chars.peek() {
                        Some(&']') => {
                            chars.next();
                            break;
                        }
                        Some(&'.') => {
                            // pos <<= 1;
                            chars.next();
                        }
                        Some(&'#') => {
                            // d |= pos;
                            // pos <<= 1;
                            chars.next();
                        }
                        _ => return Err(format!("Unexpected char in lights: {}", ch).into()),
                    }
                }
            }
            '(' => {
                let mut buttons = Vec::new();
                // consume '('
                chars.next();
                let mut num_str = String::new();
                loop {
                    match chars.peek() {
                        Some(&',') => {
                            chars.next();
                            if !num_str.is_empty() {
                                buttons.push(num_str.parse()?);
                                num_str.clear();
                            }
                        }
                        Some(&')') => {
                            chars.next();
                            if !num_str.is_empty() {
                                buttons.push(num_str.parse()?);
                                switches.push(buttons);
                            }
                            break;
                        }
                        Some(&c) if c.is_ascii_digit() => {
                            num_str.push(c);
                            chars.next();
                        }
                        _ => return Err("Unexpected char in switch group".into()),
                    }
                }
            }
            '{' => {
                chars.next(); // consume '{'
                let mut num_str = String::new();

                loop {
                    match chars.peek() {
                        Some(&',') => {
                            chars.next();
                            if !num_str.is_empty() {
                                costs.push(num_str.parse()?);
                                num_str.clear();
                            }
                        }
                        Some(&'}') => {
                            chars.next();
                            if !num_str.is_empty() {
                                costs.push(num_str.parse()?);
                            }
                            break;
                        }
                        Some(&c) if c.is_ascii_digit() => {
                            num_str.push(c);
                            chars.next();
                        }
                        _ => return Err("Unexpected char in cost group".into()),
                    }
                }
            }
            ' ' => {
                chars.next();
            }
            _ => return Err(format!("Unexpected char: {}", ch).into()),
        }
    }
    Ok((switches, costs))
}

pub fn parser_bits(path: &str) -> Result<Vec<(u16, Vec<u16>, Vec<u16>)>, Box<dyn Error>> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Unable to read path: {}\nError: {:?}", path, e))?;

    let mut requirements = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match parse_bit_line(line) {
            Ok(v) => {
                requirements.push(v);
            }
            Err(e) => {
                return Err(format!("Error occurred when parsing line {e:?}").into());
            }
        }
    }

    Ok(requirements)
}

pub fn parser_ints(path: &str) -> Result<Vec<(Vec<Vec<u16>>, Vec<u16>)>, Box<dyn Error>> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Unable to read path: {}\nError: {:?}", path, e))?;

    let mut requirements = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match parse_int_line(line) {
            Ok(v) => {
                requirements.push(v);
            }
            Err(e) => {
                return Err(format!("Error occurred when parsing line {e:?}").into());
            }
        }
    }

    Ok(requirements)
}
