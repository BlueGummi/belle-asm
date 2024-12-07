use crate::Error::{ExpectedArgument, InvalidSyntax, UnknownCharacter};
use crate::{CONFIG, SUBROUTINE_MAP, Tip, Token};
use std::process;

pub struct Lexer<'a> {
    location: u32,
    line_number: u32,
    tokens: Vec<Token>,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    #[must_use] pub fn new(line: &'a str, line_number: u32) -> Self {
        Self {
            location: 1,
            line_number,
            tokens: Vec::new(),
            chars: line.chars().peekable(),
        }
    }

    pub fn lex(&mut self) -> &Vec<Token> {
        while let Some(c) = self.chars.next() {
            self.location += 1;
            match c {
                ' ' => {
                    continue;
                }
                '\t' => {
                    self.location += 3;
                    continue;
                }
                '\n' => self.tokens.push(Token::NewLine),
                ',' => {
                    self.location += 1;
                    self.tokens.push(Token::Comma);
                }
                ';' => break,
                '&' => {
                    self.location += 1;
                    self.lex_pointer(c);
                }
                '%' => {
                    self.location += 1;
                    self.lex_register(c);
                }
                '@' => self.lex_subroutine_call(),
                'a'..='z' | 'A'..='Z' => self.lex_identifier(c),
                '#' => {
                    self.location += 1;
                    self.lex_literal(c);
                }
                '$' => {
                    self.location += 1;
                    self.lex_memory_address(c);
                }
                '.' => self.lex_label(),
                _ => {
                    UnknownCharacter(
                        c.to_string().as_str(),
                        self.line_number,
                        Some(self.location),
                    )
                    .perror();
                    Tip::Maybe("meant a numeric literal, such as #42").display_tip();
                    process::exit(1);
                }
            }
            //
        }
        &self.tokens
    }

    fn lex_pointer(&mut self, c: char) {
        let mut pointer = String::new();
        pointer.push(c);

        let is_reg = match self.chars.peek() {
            Some(&'r' | &'R') => {
                self.location += 1;
                pointer.push(self.chars.next().unwrap());

                true
            }
            Some(&'$') => {
                self.location += 1;
                pointer.push(self.chars.next().unwrap());

                false
            }
            _ => {
                ExpectedArgument(
                    "expected 'r' or '$' after '&'",
                    self.line_number,
                    Some(self.location),
                )
                .perror();
                Tip::Try("change it to something like &r0 or &$50").display_tip();
                process::exit(1);
            }
        };

        while let Some(&next) = self.chars.peek() {
            if next.is_ascii_digit() {
                pointer.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        if is_reg {
            self.handle_register(pointer);
        } else {
            self.handle_memory(pointer);
        }
    }

    fn handle_register(&mut self, pointer: String) {
        if pointer.len() > 2 {
            if let Ok(reg) = pointer.trim()[2..].parse::<i16>() {
                self.tokens.push(Token::RegPointer(reg));
            } else {
                InvalidSyntax(
                    "invalid register number",
                    self.line_number,
                    Some(self.location),
                )
                .perror();
                Tip::Try("reduce the register number").display_tip();
                process::exit(1);
            }
        } else {
            InvalidSyntax(
                "register must have a number",
                self.line_number,
                Some(self.location),
            )
            .perror();
            Tip::Try("change the value after 'r' to a numeric value under 8").display_tip();
            process::exit(1);
        }
    }

    fn handle_memory(&mut self, pointer: String) {
        if pointer.len() > 2 {
            if let Ok(mem) = pointer.trim()[2..].parse::<i16>() {
                if pointer.trim()[2..].parse::<i16>().unwrap() < 512 {
                    self.tokens.push(Token::MemPointer(mem));
                } else {
                    InvalidSyntax(
                        "address must be between 0-512",
                        self.line_number,
                        Some(self.location),
                    )
                    .perror();
                    Tip::Try("reduce the address").display_tip();
                    process::exit(1);
                }
            } else {
                InvalidSyntax(
                    "invalid memory number",
                    self.line_number,
                    Some(self.location),
                )
                .perror();
                Tip::NoIdea("this statement should be unreachable").display_tip();
                process::exit(1);
            }
        } else {
            InvalidSyntax(
                "memory must have a number",
                self.line_number,
                Some(self.location),
            )
            .perror();
            Tip::Maybe("meant to write a register or pointer").display_tip();
            process::exit(1);
        }
    }

    fn lex_register(&mut self, c: char) {
        let mut reg = String::new();
        reg.push(c);

        if let Some(&next) = self.chars.peek() {
            if next == 'r' || next == 'R' {
                reg.push(self.chars.next().unwrap());
            } else {
                ExpectedArgument(
                    "expected 'r' or 'R' after '%'",
                    self.line_number,
                    Some(self.location),
                )
                .perror();
                Tip::Try("change the character after % to r").display_tip();
                process::exit(1);
            }
        } else {
            ExpectedArgument(
                "expected register identifier after '%'",
                self.line_number,
                Some(self.location),
            )
            .perror();
            Tip::Maybe("meant something else? there doesn't appear to be anything after the '%'")
                .display_tip();
            process::exit(1);
        }

        while let Some(&next) = self.chars.peek() {
            if next.is_ascii_digit() {
                reg.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        if reg.len() > 2 {
            if let Ok(reg_num) = reg.trim()[2..].parse::<i16>() {
                self.tokens.push(Token::Register(reg_num));
            } else {
                InvalidSyntax(
                    "invalid register number",
                    self.line_number,
                    Some(self.location),
                )
                .perror();
                Tip::Maybe("didn't mean to write a register?").display_tip();
                process::exit(1);
            }
        } else {
            InvalidSyntax(
                "register must have a number",
                self.line_number,
                Some(self.location),
            )
            .perror();
            Tip::Try("put a number after the 'r'").display_tip();
            process::exit(1);
        }
    }

    fn lex_subroutine_call(&mut self) {
        let mut subroutine_call = String::new();
        while let Some(&next) = self.chars.peek() {
            if next.is_alphanumeric() || next == '_' {
                subroutine_call.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        self.tokens.push(Token::SRCall(subroutine_call));
    }

    fn lex_identifier(&mut self, c: char) {
        let mut ident = String::new();
        ident.push(c);
        while let Some(&next) = self.chars.peek() {
            if next.is_alphanumeric() || next == '_' {
                ident.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        if let Some(&next) = self.chars.peek() {
            if next == ':' {
                self.chars.next();
                return;
            }
        }
        self.tokens.push(Token::Ident(ident));
    }

    fn lex_literal(&mut self, c: char) {
        let mut number = c.to_string();
        if let Some(&next) = self.chars.peek() {
            if next == '-' {
                number.push(self.chars.next().unwrap());
            }
        }

        while let Some(&next) = self.chars.peek() {
            if next.is_ascii_digit() {
                number.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        let num_value = if let Ok(value) = number[1..].parse::<i16>() { value } else {
            InvalidSyntax(
                "value after # must be a numeric literal",
                self.line_number,
                Some(self.location),
            )
            .perror();
            process::exit(1);
        };

        if !(-128..=127).contains(&num_value) {
            InvalidSyntax(
                "numeric literal cannot be over +/- 128",
                self.line_number,
                Some(self.location),
            )
            .perror();
            process::exit(1);
        }

        let stored_value = if num_value < 0 {
            let positive_value = num_value.unsigned_abs() as u8;
            (positive_value & 0x7F) | 0x80
        } else {
            num_value as u8
        };
        self.tokens.push(Token::Literal(i16::from(stored_value)));
    }

    fn lex_memory_address(&mut self, c: char) {
        let mut addr = c.to_string();
        while let Some(&next) = self.chars.peek() {
            if next.is_ascii_digit() {
                addr.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        if addr[1..].parse::<i16>().is_err() {
            InvalidSyntax(
                "value after $ must be numeric, 0-511",
                self.line_number,
                Some(self.location),
            )
            .perror();
            process::exit(1);
        }

        let addr_val = addr[1..].parse::<i16>().unwrap();
        if addr_val >= 512 || addr_val <= 0 {
            InvalidSyntax(
                "address must be between 0-512",
                self.line_number,
                Some(self.location),
            )
            .perror();
            process::exit(1);
        }
        self.tokens.push(Token::MemAddr(addr_val));
    }

    fn lex_label(&mut self) {
        let mut label = String::new();
        while let Some(&next) = self.chars.peek() {
            if next.is_alphanumeric() || next == '_' {
                label.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        self.tokens.push(Token::Label(label));
    }
}

pub fn print_subroutine_map() {
    let map = SUBROUTINE_MAP.lock().unwrap();
    for (name, counter) in map.iter() {
        if CONFIG.verbose | CONFIG.debug {
            println!("Subroutine: {name}, Counter: {counter}");
        }
    }
}
