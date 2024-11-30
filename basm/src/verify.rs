use crate::Error::*;
use crate::*;
use std::process;
pub fn verify(ins: &Token, arg1: Option<&Token>, arg2: Option<&Token>, line_num: u32) -> bool {
    let instructions = [
        "ADD", "HLT", "JGE", "CL", "DIV", "RET", "LD", "ST", "SWP", "JZ", "SET", "CMP", "MUL",
        "INT", "MOV",
    ];
    let raw_token = ins.get_raw().to_uppercase();

    if let Token::Ident(_) = ins {
        if instructions.contains(&raw_token.as_str()) {
            return check_instruction(&raw_token, arg1, arg2, line_num);
        }
    }

    false
}

fn check_instruction(
    raw_token: &str,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: u32,
) -> bool {
    let mut has_error = false;
    let mut err_msg: &str = "";
    // ROUND ONE
    match raw_token {
        "HLT" | "RET" => {
            check_no_arguments(arg1, arg2, raw_token, line_num);
        }
        "ADD" | "LD" | "ST" | "MOV" | "MUL" | "CMP" => {
            check_two_arguments(arg1, arg2, raw_token, line_num);
        }
        "INT" => {
            check_one_or_no_arguments(arg1, arg2, raw_token, line_num);
        }
        "CALL" | "SWP" | "jz" | "SET" | "CL" | "JGE" => {
            check_one_argument(arg1, arg2, raw_token, line_num);
        }
        _ => {
            err_msg = "instruction not covered";
            has_error = true;
        }
    }
    // there's gotta be a better way to do this...
    if has_error {
        OtherError(err_msg, line_num, None).perror();
    }

    has_error
}

fn check_no_arguments(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    instruction: &str,
    line_num: u32,
) {
    if is_arg(arg1) || is_arg(arg2) {
        ExpectedArgument(
            format!("{} requires no arguments", instruction).as_str(),
            line_num,
            None,
        )
        .perror();
        process::exit(1);
    }
}

fn check_two_arguments(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    instruction: &str,
    line_num: u32,
) {
    if !is_arg(arg1) || !is_arg(arg2) {
        ExpectedArgument(
            format!("{} requires two arguments", instruction).as_str(),
            line_num,
            None,
        )
        .perror();
        process::exit(1);
    }
}

fn check_one_or_no_arguments(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    instruction: &str,
    line_num: u32,
) {
    let args_satisfied = (is_arg(arg1) || is_arg(arg2)) || (!is_arg(arg1) && !is_arg(arg2));
    if !args_satisfied {
        ExpectedArgument(
            format!("{} requires one or no arguments", instruction).as_str(),
            line_num,
            None,
        )
        .perror();
        process::exit(1);
    }
}

fn check_one_argument(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    instruction: &str,
    line_num: u32,
) {
    if !is_arg(arg1) || is_arg(arg2) {
        ExpectedArgument(
            format!("{} requires one argument", instruction).as_str(),
            line_num,
            None,
        )
        .perror();
        std::process::exit(1);
    }
}

// this is all self-explanatory, wait till you see lex.rs
fn is_arg(tok_to_check: Option<&Token>) -> bool {
    if tok_to_check.is_none() {
        return false;
    }
    if tok_to_check.is_some() {
        return matches!(
            tok_to_check.unwrap(),
            Token::Register(_)
                | Token::Literal(_)
                | Token::SRCall(_)
                | Token::MemAddr(_)
                | Token::MemPointer(_)
                | Token::RegPointer(_)
        );
    }
    false
}
