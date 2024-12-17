use crate::Token;

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
        "ADD" => check_two_arguments(arg1, arg2, raw_token, line_num)
            .and_then(|_| verify_add_args(arg1, arg2, line_num)),
        "LD" => check_two_arguments(arg1, arg2, raw_token, line_num)
            .and_then(|_| verify_ld_args(arg1, arg2, line_num)),
        "ST" => check_two_arguments(arg1, arg2, raw_token, line_num)
            .and_then(|_| verify_st_args(arg1, arg2, line_num)),
        "MOV" => check_two_arguments(arg1, arg2, raw_token, line_num)
            .and_then(|_| verify_mov_args(arg1, arg2, line_num)),
        "MUL" => check_two_arguments(arg1, arg2, raw_token, line_num)
            .and_then(|_| verify_mul_args(arg1, arg2, line_num)),
        "CMP" => check_two_arguments(arg1, arg2, raw_token, line_num)
            .and_then(|_| verify_cmp_args(arg1, arg2, line_num)),
        "DIV" => check_two_arguments(arg1, arg2, raw_token, line_num)
            .and_then(|_| verify_div_args(arg1, arg2, line_num)),
        "INT" => check_one_or_no_arguments(arg1, arg2, raw_token, line_num)
            .and_then(|_| verify_int_args(arg1, arg2, line_num)),
        "JZ" | "JO" => check_one_argument(arg1, arg2, raw_token, line_num)
            .and_then(|_| verify_jump_args(arg1, arg2, line_num)),
        "PUSH" => check_one_argument(arg1, arg2, raw_token, line_num)
            .and_then(|_| verify_push_args(arg1, arg2, line_num)),
        "POP" => check_one_argument(arg1, arg2, raw_token, line_num)
            .and_then(|_| verify_pop_args(arg1, arg2, line_num)),
        "JMP" => check_one_argument(arg1, arg2, raw_token, line_num)
            .and_then(|_| verify_jump_args(arg1, arg2, line_num)),
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

fn verify_add_args(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: u32,
) -> Result<(), String> {
    if !is_register(arg1) {
        return Err(format!(
            "ADD requires LHS to be a Register at line {}",
            line_num
        ));
    }
    if !is_register(arg2)
        && !is_literal(arg2)
        && !is_register_pointer(arg2)
        && !is_memory_address_pointer(arg2)
    {
        return Err(format!("ADD requires RHS to be a Register, literal, register pointer, or memory address pointer at line {}", line_num));
    }
    Ok(())
}

fn verify_ld_args(arg1: Option<&Token>, arg2: Option<&Token>, line_num: u32) -> Result<(), String> {
    if !is_register(arg1) {
        return Err(format!(
            "LD requires LHS to be a Register at line {}",
            line_num
        ));
    }
    if !is_memory_address(arg2) {
        return Err(format!(
            "LD requires RHS to be a Memory address at line {}",
            line_num
        ));
    }
    Ok(())
}

fn verify_st_args(arg1: Option<&Token>, arg2: Option<&Token>, line_num: u32) -> Result<(), String> {
    if !is_register_pointer(arg1) && !is_memory_address(arg1) {
        return Err(format!(
            "ST requires LHS to be a Register pointer or Memory address at line {}",
            line_num
        ));
    }
    if !is_register(arg2) {
        return Err(format!(
            "ST requires RHS to be a Register at line {}",
            line_num
        ));
    }
    Ok(())
}

fn verify_mov_args(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: u32,
) -> Result<(), String> {
    if !is_register(arg1) {
        return Err(format!(
            "MOV requires LHS to be a Register at line {}",
            line_num
        ));
    }
    if !is_register(arg2)
        && !is_literal(arg2)
        && !is_register_pointer(arg2)
        && !is_memory_address_pointer(arg2)
    {
        return Err(format!("MOV requires RHS to be a Register, literal, register pointer, or memory address pointer at line {}", line_num));
    }
    Ok(())
}

fn verify_mul_args(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: u32,
) -> Result<(), String> {
    if !is_register(arg1) {
        return Err(format!(
            "MUL requires LHS to be a Register at line {}",
            line_num
        ));
    }
    if !is_register(arg2)
        && !is_literal(arg2)
        && !is_register_pointer(arg2)
        && !is_memory_address_pointer(arg2)
    {
        return Err(format!("MUL requires RHS to be a Register, literal, register pointer, or memory address pointer at line {}", line_num));
    }
    Ok(())
}

fn verify_cmp_args(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: u32,
) -> Result<(), String> {
    if !is_register(arg1) {
        return Err(format!(
            "CMP requires LHS to be a Register at line {}",
            line_num
        ));
    }
    if !is_register(arg2)
        && !is_literal(arg2)
        && !is_register_pointer(arg2)
        && !is_memory_address_pointer(arg2)
    {
        return Err(format!("CMP requires RHS to be a Register, literal, register pointer, or memory address pointer at line {}", line_num));
    }
    Ok(())
}

fn verify_div_args(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: u32,
) -> Result<(), String> {
    if !is_register(arg1) {
        return Err(format!(
            "DIV requires LHS to be a Register at line {}",
            line_num
        ));
    }
    if !is_register(arg2)
        && !is_literal(arg2)
        && !is_register_pointer(arg2)
        && !is_memory_address_pointer(arg2)
    {
        return Err(format!("DIV requires RHS to be a Register, literal, register pointer, or memory address pointer at line {}", line_num));
    }
    Ok(())
}

fn verify_int_args(arg1: Option<&Token>, _: Option<&Token>, line_num: u32) -> Result<(), String> {
    if !is_literal(arg1) {
        return Err(format!(
            "INT requires SRC to be a Literal at line {}",
            line_num
        ));
    }
    Ok(())
}

fn verify_push_args(arg1: Option<&Token>, _: Option<&Token>, line_num: u32) -> Result<(), String> {
    if !is_register(arg1) && !is_literal(arg1) {
        return Err(format!(
            "PUSH requires SRC to be a Register or Literal at line {}",
            line_num
        ));
    }
    Ok(())
}

fn verify_pop_args(arg1: Option<&Token>, _: Option<&Token>, line_num: u32) -> Result<(), String> {
    if !is_register(arg1) {
        return Err(format!(
            "POP requires DEST to be a Register at line {}",
            line_num
        ));
    }
    Ok(())
}

fn verify_jump_args(arg1: Option<&Token>, _: Option<&Token>, line_num: u32) -> Result<(), String> {
    if !is_register_pointer(arg1) && !is_memory_address(arg1) && !is_srcall(arg1) {
        return Err(format!(
            "JMP/JZ/JO requires DEST to be a Register pointer or Memory address at line {}",
            line_num
        ));
    }
    Ok(())
}

fn is_register(tok: Option<&Token>) -> bool {
    tok.map_or(false, |tok| matches!(tok, Token::Register(_)))
}

fn is_literal(tok: Option<&Token>) -> bool {
    tok.map_or(false, |tok| matches!(tok, Token::Literal(_)))
}

fn is_memory_address(tok: Option<&Token>) -> bool {
    tok.map_or(false, |tok| matches!(tok, Token::MemAddr(_)))
}

fn is_memory_address_pointer(tok: Option<&Token>) -> bool {
    tok.map_or(false, |tok| matches!(tok, Token::MemPointer(_)))
}

fn is_register_pointer(tok: Option<&Token>) -> bool {
    tok.map_or(false, |tok| matches!(tok, Token::RegPointer(_)))
}

fn is_srcall(tok: Option<&Token>) -> bool {
    tok.map_or(false, |tok| matches!(tok, Token::SRCall(_)))
}
