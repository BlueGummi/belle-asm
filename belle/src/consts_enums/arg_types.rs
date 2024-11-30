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
    CL(Argument),
    DIV(Argument, Argument),
    RET,
    LD(Argument, Argument),
    ST(Argument, Argument),
    SWP(Argument, Argument),
    JZ(Argument),
    CMP(Argument, Argument),
    MUL(Argument, Argument),
    SET(Argument),
    INT(Argument),
    MOV(Argument, Argument),
}
/*
 *
pub const HLT_OP: i16 = 0b0000; // we need this
pub const ADD_OP: i16 = 0b0001; // we also need this
pub const JGE_OP: i16 = 0b0010; // maybe optional ?
pub const CL_OP: i16 = 0b0011; // maybe optional ?
pub const DIV_OP: i16 = 0b0100; // we need this
pub const RET_OP: i16 = 0b0101; // we need this
pub const LD_OP: i16 = 0b0110; // we need this
pub const ST_OP: i16 = 0b0111; // we need this
pub const SWP_OP: i16 = 0b1000; // we need this
pub const JZ_OP: i16 = 0b1001; // maybe optional ?
pub const CMP_OP: i16 = 0b1010; // we need this
pub const MUL_OP: i16 = 0b1011; // we need this
pub const SET_OP: i16 = 0b1100; // we need this
pub const INT_OP: i16 = 0b1101; // we need this
pub const MOV_OP: i16 = 0b1110; // we need this
pub const SR_OP: i16 = 0b1111; // we need this
*/
// ok I gotta figure out what to do here
