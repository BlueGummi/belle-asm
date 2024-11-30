use crate::*;
use colored::*;
pub enum UnrecoverableError {
    // segfaults, illegal instructions, divide by 0
    SegmentationFault(u16, Option<String>), // first one is the state of the PC, second is specifically
    IllegalInstruction(u16, Option<String>), // what happened (only prints with dbg)
    DivideByZero(u16, Option<String>),
    InvalidRegister(u16, Option<String>),
}

pub enum RecoverableError {
    UnknownFlag(u16, Option<String>), // Recoverable error: unknown flag (print in yellow)
}
impl UnrecoverableError {
    pub fn err(&self) {
        if CONFIG.quiet && !CONFIG.dont_crash {
            std::process::exit(1);
        }
        if CONFIG.quiet && CONFIG.dont_crash {
            return;
        }
        eprint!("{} ", "UNRECOVERABLE ERROR:".red());
        let err_type = match self {
            UnrecoverableError::SegmentationFault(_, _) => "Segmentation fault",
            UnrecoverableError::IllegalInstruction(_, _) => "Illegal instruction",
            UnrecoverableError::DivideByZero(_, _) => "Divide by zero",
            UnrecoverableError::InvalidRegister(_, _) => "Invalid register",
            //_ => unreachable!(),
        };
        let msg = match self {
            UnrecoverableError::SegmentationFault(_, s) => s,
            UnrecoverableError::IllegalInstruction(_, s) => s,
            UnrecoverableError::DivideByZero(_, s) => s,
            UnrecoverableError::InvalidRegister(_, s) => s,
            //_ => unreachable!(),
        };
        let location = match self {
            UnrecoverableError::SegmentationFault(n, _) => n,
            UnrecoverableError::IllegalInstruction(n, _) => n,
            UnrecoverableError::DivideByZero(n, _) => n,
            UnrecoverableError::InvalidRegister(n, _) => n,
            //_ => unreachable!(),
        };
        if !CONFIG.verbose && !CONFIG.debug {
            // default error printing
            eprintln!("{}", err_type.bold().red());
        }
        if CONFIG.verbose || CONFIG.debug {
            eprint!("{}", err_type.yellow());
            if let Some(s) = msg {
                eprint!(": {}", s.magenta());
            }
            eprintln!(": at memory address {}", location.to_string().green());
        }
        if !CONFIG.dont_crash {
            eprintln!("{}", "CRASHING...".red());
            std::process::exit(1);
        }
    }
}
// keep working on this
impl RecoverableError {
    pub fn err(&self) {
        if CONFIG.quiet {
            return;
        }
        eprint!("{} ", "RECOVERABLE ERROR:".yellow());
        let err_type = match self {
            RecoverableError::UnknownFlag(_, _) => "Unknown flag",
        };

        let msg = match self {
            RecoverableError::UnknownFlag(_, s) => s,
        };

        let location = match self {
            RecoverableError::UnknownFlag(n, _) => n,
        };
        eprint!("{}", err_type.yellow());
        if let Some(s) = msg {
            eprint!(": {}", s.magenta());
            if !CONFIG.verbose {
                println!();
            }
        }
        if CONFIG.verbose {
            eprintln!(": at memory address {}", location.to_string().green());
        }
    }
}
