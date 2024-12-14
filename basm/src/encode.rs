use crate::consts_enums::Error::{InvalidSyntax, NonexistentData};
use crate::{
    Tip, Token, ADD_OP, CMP_OP, DIV_OP, HLT_OP, INT_OP, JMP_OP, JO_OP, JZ_OP, LD_OP, MOV_OP,
    MUL_OP, NOP_OP, POP_OP, PUSH_OP, RET_OP, ST_OP,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::process;
use std::sync::Mutex;

pub static SUBROUTINE_MAP: Lazy<Mutex<HashMap<String, u32>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static MEMORY_COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));
static START_LOCATION: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));
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
        Some(Token::SR(sr) | Token::SRCall(sr)) => {
            let map = SUBROUTINE_MAP.lock().unwrap();
            if let Some(&address) = map.get(sr) {
                address as i16
            } else {
                NonexistentData(
                    format!("subroutine \"{sr}\" does not exist").as_str(),
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

#[must_use]
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
            "JO" => {
                ins_type = "one_arg";
                if let Some(&Token::SRCall(_)) = arg1.or(arg2) {
                    // handle subroutine call
                    ins_type = "call";
                } else if let Some(&Token::RegPointer(_)) = arg1.or(arg2) {
                    ins_type = "jwr";
                }

                JO_OP // 2
            }
            "POP" => {
                ins_type = "one_arg";
                POP_OP // 3
            }
            "DIV" => DIV_OP, // 4
            "RET" => RET_OP, // 5
            "LD" => LD_OP,   // 6
            "ST" => {
                if let Some(&Token::RegPointer(_)) = arg1.or(arg2) {
                    ins_type = "sti";
                } else {
                    ins_type = "st";
                }
                ST_OP // 7
            }
            "JMP" => {
                ins_type = "one_arg";
                if let Some(&Token::SRCall(_)) = arg1.or(arg2) {
                    // handle subroutine call
                    ins_type = "call";
                } else if let Some(&Token::RegPointer(_)) = arg1.or(arg2) {
                    ins_type = "jwr";
                }
                JMP_OP
            }

            "JZ" => {
                ins_type = "one_arg";
                if let Some(&Token::SRCall(_)) = arg1.or(arg2) {
                    // handle subroutine call
                    ins_type = "call";
                } else if let Some(&Token::RegPointer(_)) = arg1.or(arg2) {
                    ins_type = "jwr";
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

    match ins_type.trim().to_lowercase().as_str() {
        "one_arg" => Some((instruction_bin << 12) | argument_to_binary(arg1, line_num)),
        "st" => Some(
            (instruction_bin << 12)
                | (argument_to_binary(arg1, line_num) << 3)
                | argument_to_binary(arg2, line_num),
        ),
        "sti" => {
            let raw = arg1?.get_raw();
            let parsed_int = raw.trim().parse::<i16>().unwrap();
            Some(
                (instruction_bin << 12)
                    | (1 << 11)
                    | (argument_to_binary(Some(&Token::Register(parsed_int)), line_num) << 7)
                    | argument_to_binary(arg2, line_num),
            )
        }
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
            let address = argument_to_binary(arg1, line_num);
            Some((instruction_bin << 12) | address)
        }
        "jwr" => {
            let raw_str = arg1?.get_raw();
            let parsed_int = raw_str.trim().parse::<i16>().unwrap(); // this cannot and will not
                                                                     // fail
            Some(
                (instruction_bin << 12)
                    | 1 << 11
                    | argument_to_binary(Some(&Token::Register(parsed_int)), line_num),
            )
        }
        _ => {
            InvalidSyntax("Instruction type not recognized", line_num, None).perror();
            Tip::NoIdea("this should be unreachable").display_tip();
            process::exit(1);
        }
    }
}

pub fn process_start(lines: &Vec<String>) {
    let mut start_number: Option<i32> = None;

    for line in lines {
        if line.trim().starts_with(".start") {
            start_number = line
                .split_whitespace()
                .nth(1)
                .and_then(|s| s.strip_prefix('$'))
                .and_then(|s| s.parse::<i32>().ok());
        }
    }

    if let Some(num) = start_number {
        let mut start_location = START_LOCATION.lock().unwrap();
        *start_location = num;
    }
}

pub fn load_subroutines(lines: &Vec<String>) {
    let mut subroutine_counter = 1 + *START_LOCATION.lock().unwrap() as u32;
    let mut subroutine_map = SUBROUTINE_MAP.lock().unwrap();

    for line in lines {
        if line.trim().is_empty()
            || line.trim_start().starts_with(';')
            || line.trim().starts_with('.')
        {
            continue;
        }

        if line.ends_with(':') {
            let subroutine_name = line.trim_end_matches(':').trim().to_string();
            subroutine_map.insert(subroutine_name, subroutine_counter);
            subroutine_counter -= 1;
        }

        subroutine_counter += 1;
    }
}
pub fn update_memory_counter() {
    let mut counter = MEMORY_COUNTER.lock().unwrap();
    *counter += 1;
}
