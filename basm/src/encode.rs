use crate::consts_enums::Error::*;
use crate::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;
use std::sync::Mutex;
pub static VARIABLE_MAP: Lazy<Mutex<HashMap<String, u32>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
pub fn argument_to_binary(arg: Option<&Token>, line_num: u32) -> i16 {
    // this is some really stupid stuff
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
            // we're gonna lock the hashmap to get stuff from it
            let map = SUBROUTINE_MAP.lock().unwrap();
            let subroutine_value = map.get(sr);
            // it's gonna see if it exists and return the key if it does
            if let Some(value) = subroutine_value {
                *value as i16
            } else {
                /*
                 * now this is the real buffoonery
                 * we will read from the input file, and load every subroutine we see into a
                 * hashmap (which should be identical to the big public one) to allow for
                 * subroutine hoisting, essentially.
                 * */
                let mut subroutine_counter = 1;
                let mut subroutine_map = HashMap::new();
                let file: &String = &CONFIG.file;
                let file_result = File::open(Path::new(file));
                for line in io::BufReader::new(file_result.unwrap())
                    .lines()
                    .map_while(Result::ok)
                // this is utterly ludicrous
                {
                    let line = line.split(';').next().unwrap_or(&line); // delete comments
                    if line.ends_with(':') {
                        // add this to the hashmap for subroutines
                        let subroutine_name = line.trim_end_matches(':').trim().to_string();
                        subroutine_map.insert(subroutine_name, subroutine_counter); // what on
                                                                                    // earth am i
                                                                                    // doing
                        subroutine_counter += 1;
                    }
                }
                if !subroutine_map.contains_key(sr) {
                    NonexistentData(
                        format!("subroutine \"{}\" does not exist", sr).as_str(),
                        line_num,
                        None,
                    )
                    .perror();
                    Tip::Maybe("misspelled it?").display_tip();
                    process::exit(1);
                }

                return *subroutine_map.get(sr).unwrap_or(&0); // all this for a single number
            }
        }
        Some(Token::MemAddr(n)) => *n,
        Some(Token::Label(keyword)) => {
            let label_val: i16 = match keyword.as_str() {
                "start" => 1,
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
) -> i16 {
    // these booleans will define instruction encoding once set to true
    // different instructions are encoded differently
    /*  Standard instruction encoding, e.g. ADD, MUL
     *  Let's take ADD as our example
     *  0001 111 1 00101001 <- these last 8 bits are a num, if the first bit is on it is neg.
     *  ^    ^   ^This standalone '1' is the determinant bit, if it is on, the next value is
     *  |    |    a literal, if off, it is a register
     *  |    |
     *  |    These next 3 bits are the destination register, max value of 7
     *  These first four bits denote the opcode. opcodes are always 4 bits long.
     */

    /* now let's look at the encoding for an instruction such as ST, storing a value from register
     * to memory
     * 0111 1101 10101 001
     * ^    ^          ^ Last 3 bits denote SOURCE register.
     * |    |These 9 bits denote a memory address, max 512
     * | Opcode
     */

    // for other instructions, such as RET and HLT, I should implement variants with arguments to
    // denote things such as the .start keyword, like 00001000 VALUE000
    let mut ins_type = "default";
    let instruction_bin = match ins {
        // first one will always be an instruction
        Token::Ident(ref instruction) => match instruction.to_uppercase().as_str() {
            // self
            // explanatory
            "HLT" => HLT_OP, // 0
            "ADD" => ADD_OP, // 1
            "JGE" => {
                ins_type = "one_arg";
                if let Some(&Token::SRCall(_)) = arg1.or(arg2) {
                    // do something here
                    ins_type = "call";
                }
                JGE_OP // 2
            }
            "CL" => {
                ins_type = "one_arg";
                CL_OP // 3
            }
            "DIV" => {
                DIV_OP // 4
            }
            "RET" => RET_OP, // 5
            "LD" => LD_OP,   // 6
            // I feel like this LD instruction may be unnecessary?
            // but maybe not, MOV can't handle large mem addrs
            "ST" => {
                ins_type = "st";
                ST_OP // 7
            }
            "SWP" => {
                SWP_OP // 8
            }
            "JNZ" => {
                ins_type = "one_arg";
                if let Some(&Token::SRCall(_)) = arg1.or(arg2) {
                    // do something here
                    ins_type = "call";
                }
                // we can use this as JMP by doing cmp r0, r0
                JNZ_OP // 9
            }
            "CMP" => CMP_OP, // 10
            "MUL" => MUL_OP, // 11
            // mul mainly exists because MOV and LD and ST cannot handle big numbers
            "SET" => {
                ins_type = "one_arg";
                SET_OP // 12
            }
            "INT" => {
                ins_type = "one_arg";
                INT_OP // 13
            }
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
            if CONFIG.debug {
                println!("Subroutine detected");
            }
            SR_OP
        }
        Token::Label(_) => {
            ins_type = "label";
            if CONFIG.debug {
                println!("Keyword detected");
            }
            HLT_OP
        }
        _ => {
            InvalidSyntax("invalid instruction type", line_num, None).perror();
            Tip::NoIdea("yeah I have no idea how you got here").display_tip();
            process::exit(1);
        }
    };
    // these are the last statements so we don't type return
    match ins_type.trim().to_lowercase().as_str() {
        "subr" => (instruction_bin << 12) | argument_to_binary(Some(ins), line_num),
        "one_arg" => (instruction_bin << 12) | argument_to_binary(arg1, line_num),
        "st" => {
            (instruction_bin << 12)
                | (argument_to_binary(arg1, line_num) << 3)
                | argument_to_binary(arg2, line_num)
        }
        "label" => {
            (instruction_bin << 12)
                | (argument_to_binary(Some(ins), line_num) << 9)
                | argument_to_binary(arg1, line_num)
        }
        "default" => {
            let arg1_bin = argument_to_binary(arg1, line_num);
            let arg2_bin = argument_to_binary(arg2, line_num);
            (instruction_bin << 12) | (arg1_bin << 9) | arg2_bin
        }
        "call" => (instruction_bin << 12) | 1 << 11 | argument_to_binary(arg1, line_num),
        _ => {
            InvalidSyntax("Instruction type not recognized", line_num, None).perror();
            Tip::NoIdea("this should be unreachable").display_tip();
            process::exit(1);
        }
    }
}
