use crate::{CONFIG, CPU_STATE};
use colored::Colorize;

pub enum UnrecoverableError {
    SegmentationFault(u16, Option<String>),
    IllegalInstruction(u16, Option<String>),
    DivideByZero(u16, Option<String>),
    InvalidRegister(u16, Option<String>),
    StackOverflow(u16, Option<String>),
    StackUnderflow(u16, Option<String>),
}

pub enum RecoverableError {
    UnknownFlag(u16, Option<String>),
    Overflow(u16, Option<String>),
    BackwardStack(u16, Option<String>),
}

impl UnrecoverableError {
    pub fn err(&self) {
        if CONFIG.quiet {
            std::process::exit(1);
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
            self.debug_info(location);
        }

        eprintln!("{}", "CRASHING...".red());
        std::process::exit(1);
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

    fn debug_info(&self, location: u16) {
        let state = CPU_STATE.lock().unwrap();
        if let Some(cpu) = state.values().find(|cpu| cpu.pc == location) {
            if let Some(data) = cpu.memory[location as usize] {
                eprintln!("Instruction is {}", format!("{data:016b}").magenta());
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
