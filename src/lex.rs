use crate::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
// really goofy
pub static SUBROUTINE_MAP: Lazy<Mutex<HashMap<String, u32>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static SUBROUTINE_COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(1));

pub fn print_subroutine_map() {
    let map = SUBROUTINE_MAP.lock().unwrap();
    for (name, counter) in map.iter() {
        if CONFIG.verbose | CONFIG.debug {
            println!("Subroutine: {}, Counter: {}", name, counter);
        }
    }
}

pub fn lex(line: &str, line_number: u32) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = line.chars().peekable(); // iterator

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' => continue,
            '\n' => tokens.push(Token::NewLine),
            ',' => tokens.push(Token::Comma),
            ';' => {
                tokens.push(Token::Semicolon);
                break;
            }
            '\"' => {
                let mut value = String::new();
                while let Some(&next) = chars.peek() {
                    if next != '\"' {
                        value.push(chars.next().unwrap()); // keep adding on
                    } else {
                        break;
                    }
                }
                if let Some(&next) = chars.peek() {
                    if next == '\"' {
                        tokens.push(Token::Value(value)); // just push the ident
                    }
                }
            }
            '&' => {
                let mut pointer = String::new();
                pointer.push(c);
                let mut is_reg: bool = true;
                if let Some(&next) = chars.peek() {
                    match next {
                        'r' | 'R' => {
                            pointer.push(chars.next().unwrap());
                        }
                        '$' => {
                            pointer.push(chars.next().unwrap());
                            is_reg = false;
                        }
                        _ => {
                            eprintln!("Expected 'r', 'R', or '$' after '&': line {}", line_number);
                            std::process::exit(1);
                        }
                    }
                }
                while let Some(&next) = chars.peek() {
                    // read after reg/mem
                    if next.is_ascii_digit() {
                        pointer.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if is_reg {
                    if pointer.len() > 2 {
                        if let Ok(reg) = pointer.trim()[2..].parse::<i16>() {
                            tokens.push(Token::RegPointer(reg));
                        } else {
                            eprintln!("Invalid register number: line {}", line_number);
                            break;
                        }
                    } else {
                        eprintln!("Register must have a number: line {}", line_number);
                        break;
                    }
                }
                if !is_reg {
                    if pointer.len() > 2 {
                        if let Ok(mem) = pointer.trim()[2..].parse::<i16>() {
                            tokens.push(Token::MemPointer(mem));
                        } else {
                            eprintln!("Invalid memory number: line {}", line_number);
                            break;
                        }
                    } else {
                        eprintln!("Memory must have a number: line {}", line_number);
                        break;
                    }
                }
            }

            '%' => {
                let mut reg = String::new();
                reg.push(c);
                if let Some(&next) = chars.peek() {
                    if next == 'r' || next == 'R' {
                        reg.push(chars.next().unwrap()); // keep going
                    } else {
                        eprintln!("Expected 'r' or 'R' after '%': line {}", line_number);
                        break;
                    }
                } else {
                    eprintln!(
                        "Expected register identifier after '%': line {}",
                        line_number
                    );
                    break;
                }
                while let Some(&next) = chars.peek() {
                    // read after 'r'
                    if next.is_ascii_digit() {
                        reg.push(chars.next().unwrap()); // push to str
                    } else {
                        break;
                    }
                }
                if reg.len() > 2 {
                    // make sure it long enough
                    if let Ok(reg_num) = reg.trim()[2..].parse::<i16>() {
                        // parse the #
                        tokens.push(Token::Register(reg_num));
                    } else {
                        eprintln!("Invalid register number: line {}", line_number);
                        break;
                    }
                } else {
                    eprintln!("Register must have a number: line {}", line_number);
                    break;
                }
            }
            '@' => {
                let mut subroutine_call = String::new(); // make a string
                while let Some(&next) = chars.peek() {
                    // iterator again
                    if next.is_alphanumeric() || next == '_' {
                        // yay sub_rou_tines
                        subroutine_call.push(chars.next().unwrap()); // push it to a string
                    } else {
                        break;
                    }
                } // check again and again and then push it
                tokens.push(Token::SRCall(subroutine_call));
            }
            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::new();
                ident.push(c); // push first letter
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' {
                        ident.push(chars.next().unwrap()); // keep adding on
                    } else {
                        break;
                    }
                }
                if let Some(&next) = chars.peek() {
                    if next == ':' {
                        // subroutines
                        chars.next(); // get the next thing
                        let mut map = SUBROUTINE_MAP.lock().unwrap();
                        if !map.contains_key(&ident) {
                            let mut counter = SUBROUTINE_COUNTER.lock().unwrap();
                            map.insert((*ident).to_string(), *counter); // get that ident
                            *counter += 1;
                            tokens.push(Token::SR(ident)); // add the ident
                        } else {
                            eprintln!(
                                "Duplicate subroutine declaration: '{}': line {}",
                                ident, line_number
                            );
                        }
                    } else {
                        tokens.push(Token::Ident(ident)); // just push the ident
                    }
                } else {
                    tokens.push(Token::Ident(ident));
                }
            }
            '#' => {
                let mut number = c.to_string();
                if let Some(&next) = chars.peek() {
                    if next == '-' {
                        number.push(chars.next().unwrap());
                    }
                } // negative numbers (runs once)

                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                } // keep going and going

                let num_value = match number[1..].parse::<i16>() {
                    // parse second digit to end
                    Ok(value) => value,
                    Err(_) => {
                        eprintln!(
                            "Value after # must be numeric literal: line {}",
                            line_number
                        );
                        eprintln!("{}", number);
                        std::process::exit(1);
                    }
                };

                if !(-128..=128).contains(&num_value) {
                    eprintln!(
                        "Numeric literal cannot be over +/- 128: line {}",
                        line_number
                    );
                    std::process::exit(1);
                }

                // if it is less than 0, bitflip first bit
                let stored_value = if num_value < 0 {
                    // first positive value as sign bit
                    let positive_value = num_value.unsigned_abs() as u8; // convert to positive
                    (positive_value & 0x7F) | 0x80 // set the sign bit (flip first bit)
                                                   // hex because lazy
                } else {
                    num_value as u8
                }; // all good

                tokens.push(Token::Literal(stored_value as i16));
            }
            '$' => {
                let mut addr = c.to_string();
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() {
                        addr.push(chars.next().unwrap());
                    } else {
                        break;
                    } // add it all to addr string
                }
                if addr[1..].parse::<i16>().is_err() {
                    // try to get an address
                    eprintln!(
                        "Value after $ must be numeric, 512: line {}", // TODO
                        line_number
                    );
                    std::process::exit(1);
                }
                let addr_val = addr[1..].parse::<i16>().unwrap();
                if addr_val >= 512 || addr_val <= 0 {
                    eprintln!("Address must be between 0-512: line {}", line_number); // TODO
                    std::process::exit(1);
                }
                tokens.push(Token::MemAddr(addr_val));
            }
            '.' => {
                let mut label = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' {
                        label.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Label(label));
            }
            _ => {
                eprintln!("Unknown character: {}: line {}", c, line_number);
            }
        }
    }

    tokens.push(Token::Eol); // finally push eol at end of line
    tokens
}
