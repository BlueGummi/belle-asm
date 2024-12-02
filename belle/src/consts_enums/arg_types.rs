use std::fmt;

pub enum Argument {
    Register(i16),
    MemAddr(i16),
    Literal(i16),
    RegPtr(i16),
    MemPtr(i16),
    SR(i16),
    Flag(i16),
    Nothing,
}

pub enum Instruction {
    HLT,
    ADD(Argument, Argument),
    JGE(Argument),
    POP(Argument),
    DIV(Argument, Argument),
    RET,
    LD(Argument, Argument),
    ST(Argument, Argument),
    SWP(Argument, Argument),
    JZ(Argument),
    CMP(Argument, Argument),
    MUL(Argument, Argument),
    PUSH(Argument),
    INT(Argument),
    MOV(Argument, Argument),
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Argument::Register(val) => write!(f, "R{}", val),
            Argument::MemAddr(val) => write!(f, "[{}]", val),
            Argument::Literal(val) => write!(f, "{}", val),
            Argument::RegPtr(val) => write!(f, "RegPtr({})", val),
            Argument::MemPtr(val) => write!(f, "MemPtr({})", val),
            Argument::SR(val) => write!(f, "SR({})", val),
            Argument::Flag(val) => write!(f, "Flag({})", val),
            Argument::Nothing => write!(f, "Nothing"),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::HLT => write!(f, "HLT"),
            Instruction::ADD(arg1, arg2) => write!(f, "ADD {}, {}", arg1, arg2),
            Instruction::JGE(arg) => write!(f, "JGE {}", arg),
            Instruction::POP(arg) => write!(f, "POP {}", arg),
            Instruction::DIV(arg1, arg2) => write!(f, "DIV {}, {}", arg1, arg2),
            Instruction::RET => write!(f, "RET"),
            Instruction::LD(arg1, arg2) => write!(f, "LD {}, {}", arg1, arg2),
            Instruction::ST(arg1, arg2) => write!(f, "ST {}, {}", arg1, arg2),
            Instruction::SWP(arg1, arg2) => write!(f, "SWP {}, {}", arg1, arg2),
            Instruction::JZ(arg) => write!(f, "JZ {}", arg),
            Instruction::CMP(arg1, arg2) => write!(f, "CMP {}, {}", arg1, arg2),
            Instruction::MUL(arg1, arg2) => write!(f, "MUL {}, {}", arg1, arg2),
            Instruction::PUSH(arg) => write!(f, "PUSH {}", arg),
            Instruction::INT(arg) => write!(f, "INT {}", arg),
            Instruction::MOV(arg1, arg2) => write!(f, "MOV {}, {}", arg1, arg2),
        }
    }
}
