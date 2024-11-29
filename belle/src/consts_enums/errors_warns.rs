use colored::*;
use crate::*;
pub enum UnrecoverableError {
    // segfaults, illegal instructions, divide by 0
    SegmentationFault(u16, Option<String>), // first one is the state of the PC, second is specifically
    IllegalInstruction(u16, Option<String>), // what happened (only prints with dbg)
    DivideByZero(u16, Option<String>),
}

pub enum RecoverableError {
    // I will come back to this later, CPU time! 
    TempErr(),
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
            //_ => unreachable!(),
        };
        let msg = match self {
            UnrecoverableError::SegmentationFault(_, s) => s,
            UnrecoverableError::IllegalInstruction(_, s) => s,
            UnrecoverableError::DivideByZero(_, s) => s,
            //_ => unreachable!(),
        };
        let location = match self {
            UnrecoverableError::SegmentationFault(n, _) => n,
            UnrecoverableError::IllegalInstruction(n, _) => n,
            UnrecoverableError::DivideByZero(n, _) => n,
            //_ => unreachable!(),
        };
        if !CONFIG.verbose && !CONFIG.debug && !CONFIG.dont_crash { // default error printing
            eprintln!("{}", err_type.yellow());
        }
        if CONFIG.verbose || CONFIG.debug {
            eprint!("{}", err_type.yellow());
            if let Some(s) = msg {
                eprint!(": {}", s.magenta());
            }
            eprintln!(": at memory address {}", location.to_string().green());
        }
        if CONFIG.debug {
            // write to a file with the data in memory
            println!("Please implement a way to print the memory");
            std::process::exit(69);
        }
        if !CONFIG.dont_crash {
            eprintln!("{}", "CRASHING...".red());
            std::process::exit(1);
        }
    }
}
// keep working on this
