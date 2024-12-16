use crate::{CONFIG, CPU_STATE};
use colored::Colorize;
use std::fmt;

#[derive(Debug)]
pub enum UnrecoverableError {
    SegmentationFault(u16, Option<String>),
    IllegalInstruction(u16, Option<String>),
    DivideByZero(u16, Option<String>),
    InvalidRegister(u16, Option<String>),
    StackOverflow(u16, Option<String>),
    StackUnderflow(u16, Option<String>),
}

#[derive(Debug)]
pub enum RecoverableError {
    UnknownFlag(u16, Option<String>),
    Overflow(u16, Option<String>),
    BackwardStack(u16, Option<String>),
}

pub type Oopsie = Result<(), RecoverableError>;
pub type Death = Result<(), UnrecoverableError>; // what am I supposed to call it?

impl std::error::Error for UnrecoverableError {}
impl std::error::Error for RecoverableError {}
impl UnrecoverableError {
    pub fn err(&self) {
        if CONFIG.quiet && !CONFIG.debug {
            return;
        }

        eprint!("{} ", "UNRECOVERABLE ERROR:".red());
        let (err_type, location, msg) = self.details();

        if !CONFIG.verbose && !CONFIG.debug {
            eprintln!("{}", err_type.bold().red());
        } else {
            eprint!("{}", err_type.yellow());
            if let Some(s) = msg {
                eprint!(": {}", s.magenta());
            }
            eprintln!(": at memory address {}", location.to_string().green());
        }

        if CONFIG.debug {
            self.debug_info(&location);
        }

        eprintln!("{}", "CRASHING...".red());
    }

    fn details(&self) -> (&str, u16, &Option<String>) {
        match self {
            UnrecoverableError::SegmentationFault(loc, msg) => ("Segmentation fault", *loc, msg),
            UnrecoverableError::IllegalInstruction(loc, msg) => ("Illegal instruction", *loc, msg),
            UnrecoverableError::DivideByZero(loc, msg) => ("Divide by zero", *loc, msg),
            UnrecoverableError::InvalidRegister(loc, msg) => ("Invalid register", *loc, msg),
            UnrecoverableError::StackOverflow(loc, msg) => ("Stack overflow", *loc, msg),
            UnrecoverableError::StackUnderflow(loc, msg) => ("Stack underflow", *loc, msg),
        }
    }

    fn debug_info(&self, location: &u16) {
        let state = CPU_STATE.lock().unwrap();
        if let Some(cpu) = state.values().find(|cpu| cpu.pc == *location) {
            if let Some((_, data)) = cpu.memory.iter().find(|&&(first, _)| first == *location) {
                eprintln!("Instruction is {}", format!("{:016b}", data).magenta());
            } else {
                eprintln!(
                    "{}",
                    "No instruction found at this program counter".red().bold()
                );
            }
        }
    }
}

impl RecoverableError {
    pub fn err(&self) {
        if !CONFIG.verbose {
            return;
        }

        eprint!("{} ", "RECOVERABLE ERROR:".yellow());
        let (err_type, location, msg) = self.details();

        eprint!("{}", err_type.yellow());
        if let Some(s) = msg {
            eprint!(": {}", s.magenta());
        }
        if CONFIG.verbose {
            eprintln!(": at memory address {}", location.to_string().green());
        }
    }

    fn details(&self) -> (&str, u16, &Option<String>) {
        match self {
            RecoverableError::UnknownFlag(loc, msg) => ("Unknown flag", *loc, msg),
            RecoverableError::Overflow(loc, msg) => ("Overflow", *loc, msg),
            RecoverableError::BackwardStack(loc, msg) => ("Backwards stack", *loc, msg),
        }
    }
}

impl fmt::Display for UnrecoverableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (err_type, location, msg) = self.details();
        write!(f, "{}: ", "UNRECOVERABLE ERROR:".red())?;
        write!(f, "{}", err_type.bold().red())?;

        if let Some(s) = msg {
            write!(f, ": {}", s.magenta())?;
        }

        write!(f, ": at memory address {}", location.to_string().green())?;
        Ok(())
    }
}

impl fmt::Display for RecoverableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (err_type, location, msg) = self.details();
        write!(f, "{}: ", "RECOVERABLE ERROR:".yellow())?;
        write!(f, "{}", err_type.yellow())?;

        if let Some(s) = msg {
            write!(f, ": {}", s.magenta())?;
        }

        write!(f, ": at memory address {}", location.to_string().green())?;
        Ok(())
    }
}
