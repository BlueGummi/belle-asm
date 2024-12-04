use crate::consts_enums::Error::*;
use crate::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::{Seek, SeekFrom};
use std::path::Path;
use std::process;
use std::sync::Mutex;

pub static SUBROUTINE_MAP: Lazy<Mutex<HashMap<String, u32>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static MEMORY_COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

pub fn argument_to_binary(arg: Option<&Token>, line_num: u32) -> i16 {
    match arg {
        // register
        Some(Token::Register(num)) => {
            if *num > 8 {
                InvalidSyntax("register number cannot be greater than 7", line_num, None).perror();
                Tip::Try("change the number to a value below 7").display_tip();
                process::exit(1);
            }
            *num
        }
        // all looks good
        Some(Token::Literal(literal)) => (1 << 8) | *literal,
        Some(Token::SR(sr)) | Some(Token::SRCall(sr)) => {
            let map = SUBROUTINE_MAP.lock().unwrap();
            if let Some(&address) = map.get(sr) {
                address as i16
            } else {
                NonexistentData(
                    format!("subroutine \"{}\" does not exist", sr).as_str(),
                    line_num,
                    None,
                )
                .perror();
                Tip::Maybe("misspelled it?").display_tip();
                process::exit(1);
            }
        }
        Some(Token::MemAddr(n)) => *n,
        Some(Token::Label(keyword)) => {
            let label_val: i16 = match keyword.as_str() {
                "start" => 1,
                "ssp" => 2,
                "sbp" => 3,
                _ => {
                    InvalidSyntax("label not recognized after '.'", line_num, Some(1)).perror();
                    Tip::Try("change the label name to a valid one\nsuch as .start").display_tip();
                    std::process::exit(1);
                }
            };
            label_val
        }
        Some(Token::MemPointer(mem)) => (1 << 7) | mem,
        Some(Token::RegPointer(reg)) => (1 << 6) | reg,
        _ => 0,
    }
}

pub fn encode_instruction(
    ins: &Token,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: u32,
) -> Option<i16> {
    let mut ins_type = "default";
    let instruction_bin = match ins {
        Token::Ident(ref instruction) => match instruction.to_uppercase().as_str() {
            "HLT" => HLT_OP, // 0
            "ADD" => ADD_OP, // 1
            "JGE" => {
                ins_type = "one_arg";
                if let Some(&Token::SRCall(_)) = arg1.or(arg2) {
                    // handle subroutine call
                    ins_type = "call";
                }
                JGE_OP // 2
            }
            "POP" => {
                ins_type = "one_arg";
                POP_OP // 3
            }
            "DIV" => DIV_OP, // 4
            "RET" => RET_OP, // 5
            "LD" => LD_OP,   // 6
            "ST" => {
                ins_type = "st";
                ST_OP // 7
            }
            "SWP" => SWP_OP, // 8
            "JZ" => {
                ins_type = "one_arg";
                if let Some(&Token::SRCall(_)) = arg1.or(arg2) {
                    // handle subroutine call
                    ins_type = "call";
                }
                JZ_OP // 9
            }
            "CMP" => CMP_OP, // 10
            "MUL" => MUL_OP, // 11
            "PUSH" => {
                ins_type = "one_arg";
                PUSH_OP // 12
            }
            "INT" => {
                ins_type = "one_arg";
                INT_OP // 13
            }
            "NOP" => NOP_OP,
            "MOV" => MOV_OP, // 14
            _ => {
                InvalidSyntax("instruction not recognized", line_num, None).perror();
                Tip::Try("look at the instructions.rs file in\nsrc/consts_enums/instructions.rs")
                    .display_tip();
                process::exit(1);
            }
        },
        Token::SR(_) => {
            ins_type = "subr";
            0
        }
        Token::Label(_) => {
            ins_type = "label";
            HLT_OP
        }
        _ => {
            InvalidSyntax("invalid instruction type", line_num, None).perror();
            Tip::NoIdea("yeah I have no idea how you got here").display_tip();
            process::exit(1);
        }
    };

    // Handle instruction types and generate binary encoding
    match ins_type.trim().to_lowercase().as_str() {
        "subr" => None,
        "one_arg" => Some((instruction_bin << 12) | argument_to_binary(arg1, line_num)),
        "st" => Some(
            (instruction_bin << 12)
                | (argument_to_binary(arg1, line_num) << 3)
                | argument_to_binary(arg2, line_num),
        ),
        "label" => Some(
            (instruction_bin << 12)
                | (argument_to_binary(Some(ins), line_num) << 9)
                | argument_to_binary(arg1, line_num),
        ),
        "default" => {
            let arg1_bin = argument_to_binary(arg1, line_num);
            let arg2_bin = argument_to_binary(arg2, line_num);
            Some((instruction_bin << 12) | (arg1_bin << 9) | arg2_bin)
        }
        "call" => {
            // Handle subroutine call: replace with memory address
            let address = argument_to_binary(arg1, line_num);
            Some((instruction_bin << 12) | address)
        }
        _ => {
            InvalidSyntax("Instruction type not recognized", line_num, None).perror();
            Tip::NoIdea("this should be unreachable").display_tip();
            process::exit(1);
        }
    }
}

pub fn load_subroutines() {
    let file = &CONFIG.file;
    let mut file = File::open(Path::new(file)).unwrap();

    let mut subroutine_counter = 1;
    let mut subroutine_map = SUBROUTINE_MAP.lock().unwrap();

    let reader = io::BufReader::new(&mut file);
    for line in reader.lines() {
        subroutine_counter += 1;
        let line = match line {
            Ok(line) => line,
            Err(_) => continue,
        };
        if line.trim().is_empty()
            || line.trim_start().starts_with(';')
            || line.trim().starts_with('.')
        {
            subroutine_counter -= 1;
            continue;
        }
        if line.ends_with(':') {
            subroutine_counter -= 1;
            let subroutine_name = line.trim_end_matches(':').trim().to_string();
            subroutine_map.insert(subroutine_name, subroutine_counter);
        }
    }

    file.seek(SeekFrom::Start(0)).unwrap();

    std::mem::drop(subroutine_map);

    let reader = io::BufReader::new(&mut file);
    for line in reader.lines().map_while(Result::ok) {
        if line.trim().starts_with(".start") {
            if let Some(start_number) = line
                .split_whitespace()
                .nth(1)
                .and_then(|s| s.strip_prefix('$'))
                .and_then(|s| s.parse::<i32>().ok())
            {
                let mut subroutine_map = SUBROUTINE_MAP.lock().unwrap();
                for value in subroutine_map.values_mut() {
                    *value += start_number as u32;
                }
            }
        }
    }
}

pub fn update_memory_counter() {
    let mut counter = MEMORY_COUNTER.lock().unwrap();
    *counter += 1; // Increment memory address after encoding an instruction
}
