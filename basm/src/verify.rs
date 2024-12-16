use crate::Token;

#[must_use]
pub fn verify(
    ins: &Token,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: u32,
) -> Result<(), String> {
    let instructions = [
        "ADD", "HLT", "JO", "POP", "DIV", "RET", "LD", "ST", "JMP", "JZ", "PUSH", "CMP", "MUL",
        "INT", "MOV",
    ];
    let raw_token = ins.get_raw().to_uppercase();

    if let Token::Ident(_) = ins {
        if instructions.contains(&raw_token.as_str()) {
            return check_instruction(&raw_token, arg1, arg2, line_num);
        }
    }
    Ok(())
}

fn check_instruction(
    raw_token: &str,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: u32,
) -> Result<(), String> {
    match raw_token {
        "HLT" | "RET" => check_no_arguments(arg1, arg2, raw_token, line_num),
        "ADD" | "LD" | "ST" | "MOV" | "MUL" | "CMP" | "DIV" => {
            check_two_arguments(arg1, arg2, raw_token, line_num)
        }
        "INT" => check_one_or_no_arguments(arg1, arg2, raw_token, line_num),
        "JZ" | "PUSH" | "POP" | "JO" | "JMP" => check_one_argument(arg1, arg2, raw_token, line_num),
        _ => Ok(()),
    }
}

fn check_no_arguments(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    instruction: &str,
    line_num: u32,
) -> Result<(), String> {
    if is_arg(arg1) || is_arg(arg2) {
        return Err(format!(
            "{} requires no arguments at line {}",
            instruction, line_num
        ));
    }
    Ok(())
}

fn check_two_arguments(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    instruction: &str,
    line_num: u32,
) -> Result<(), String> {
    if !is_arg(arg1) || !is_arg(arg2) {
        return Err(format!(
            "{} requires two arguments at line {}",
            instruction, line_num
        ));
    }
    Ok(())
}

fn check_one_or_no_arguments(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    instruction: &str,
    line_num: u32,
) -> Result<(), String> {
    let args_satisfied = (is_arg(arg1) || is_arg(arg2)) || (!is_arg(arg1) && !is_arg(arg2));
    if !args_satisfied {
        return Err(format!(
            "{} requires one or no arguments at line {}",
            instruction, line_num
        ));
    }
    Ok(())
}

fn check_one_argument(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    instruction: &str,
    line_num: u32,
) -> Result<(), String> {
    if !is_arg(arg1) || is_arg(arg2) {
        return Err(format!(
            "{} requires one argument at line {}",
            instruction, line_num
        ));
    }
    Ok(())
}

fn is_arg(tok_to_check: Option<&Token>) -> bool {
    tok_to_check.map_or(false, |tok| {
        matches!(
            tok,
            Token::Register(_)
                | Token::Literal(_)
                | Token::SRCall(_)
                | Token::MemAddr(_)
                | Token::MemPointer(_)
                | Token::RegPointer(_)
        )
    })
}
