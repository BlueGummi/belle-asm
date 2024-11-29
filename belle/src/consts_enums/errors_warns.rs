use colored::*;
use crate::*;
pub enum UnrecoverableError {
    // segfaults, illegal instructions, divide by 0
    SegmentationFault(u16, Option<String>), // first one is the state of the PC, second is specifically
    IllegalInstruction(u16, Option<String>), // what happened (only prints with dbg)
    DivideByZero(u16, Option<String>),
}

impl UnrecoverableError {
    pub fn err(&self) {
        eprint!("{} ", "UNRECOVERABLE ERROR:".red());
        let err_type = match self {
            UnrecoverableError::SegmentationFault(_, _) => "Segmentation fault",
            UnrecoverableError::IllegalInstruction(_, s) => "Illegal instruction",
            UnrecoverableError::DivideByZero(_, s) => "Divide by zero",
            _ => unreachable!(),
        };
        let msg = match self {
            UnrecoverableError::SegmentationFault(_, s) => s,
            UnrecoverableError::IllegalInstruction(_, s) => s,
            UnrecoverableError::DivideByZero(_, s) => s,
            _ => unreachable!(),
        };
// keep working on this
