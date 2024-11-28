mod config;
mod consts_enums;
mod encode;
mod lexer;
mod verify;
pub use config::*;
pub use consts_enums::*;
pub use encode::*;
pub use lexer::*;
pub use verify::*;
#[test]
pub fn mov_check() {
    let result = encode_instruction(
        &Token::Ident("mov".to_string()),
        Some(&Token::Register(0)),
        Some(&Token::Literal(4)),
        1,
    );
    assert_eq!(result.unsigned_abs() as u16, 0b01111011111100);
} // I don't know why it's signed and unsigned sometimes
  // weird
#[test]
pub fn hlt_check() {
    let result = encode_instruction(&Token::Ident("hlt".to_string()), None, None, 1);
    assert_eq!(result, 0);
}
#[test]
pub fn add_check() {
    let result = encode_instruction(
        &Token::Ident("add".to_string()),
        Some(&Token::Register(6)),
        Some(&Token::Literal(4)),
        1,
    );
    assert_eq!(result as u16, 0b0001110100000100);
}
#[test]
pub fn jge_check() {
    let result = encode_instruction(
        &Token::Ident("jge".to_string()),
        Some(&Token::Literal(3)),
        None,
        1,
    );
    println!("rsrsr {result:b}");
    assert_eq!(result as u16, 0b0010000100000011);
}
#[test]
pub fn cl_check() {
    let result = encode_instruction(
        &Token::Ident("cl".to_string()),
        Some(&Token::Literal(2)),
        None,
        1,
    );
    assert_eq!(result.abs() as u16, 0b0011000100000010);
}
#[test]
pub fn mul_check() {
    let result = encode_instruction(
        &Token::Ident("mul".to_string()),
        Some(&Token::Register(0)),
        Some(&Token::RegPointer(7)),
        1,
    );
    assert_eq!(result as u16, 0b1011000001000111);
}

#[test]
pub fn ret_check() {
    let result = encode_instruction(&Token::Ident("ret".to_string()), None, None, 1);
    assert_eq!(result as u16, 0b0101000000000000);
}
#[test]
pub fn ld_check() {
    let result = encode_instruction(
        &Token::Ident("ld".to_string()),
        Some(&Token::Register(5)),
        Some(&Token::Literal(1)),
        1,
    );
    assert_eq!(result as u16, 0b0110101100000001);
}
#[test]
pub fn st_check() {
    let result = encode_instruction(
        &Token::Ident("st".to_string()),
        Some(&Token::MemAddr(1)),
        Some(&Token::Register(4)),
        1,
    );
    println!("st {result:b}");
    assert_eq!(result as u16, 0b0111000000001100);
    //           ^ 1 over here
}
#[test]
pub fn jz_check() {
    let result = encode_instruction(
        &Token::Ident("jnz".to_string()),
        Some(&Token::MemAddr(8)),
        None,
        1,
    );
    assert_eq!(result as u16, 0b1001000000001000);
}
#[test]
pub fn int_check() {
    let result = encode_instruction(
        &Token::Ident("int".to_string()),
        Some(&Token::Literal(1)),
        None,
        1,
    );
    assert_eq!(result as u16, 0b1101000100000001);
}
#[test]
pub fn cmp_check() {
    let result = encode_instruction(
        &Token::Ident("cmp".to_string()),
        Some(&Token::Register(4)),
        Some(&Token::MemPointer(32)),
        1,
    );
    assert_eq!(result as u16, 0b1010100010100000);
}
