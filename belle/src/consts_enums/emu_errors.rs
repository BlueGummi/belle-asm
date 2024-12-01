use crate::*;
use colored::*;
pub enum EmuError {
    FileNotFound(),
    MemoryOverflow(),
    Duplicate(String),
    ReadFail(String),
    Impossible(String),
}
impl EmuError {
    pub fn err(&self) {
        eprint!("{} ", "Emulator Error:".red());
        match self {
            EmuError::FileNotFound() => {
                eprintln!("File {} not found", CONFIG.file.to_string().green());
            }
            EmuError::MemoryOverflow() => {
                eprintln!("{}", "Memory will overflow".red());
            }
            EmuError::Duplicate(s) => {
                eprintln!("Duplicate: {}", s.red());
            }
            EmuError::ReadFail(s) => {
                eprintln!(
                    "{}: {}",
                    "Failed to read from stdin and parse to i16".red(),
                    s
                );
            }
            EmuError::Impossible(s) => {
                eprintln!("{}: {}", "Configuration combination not possible".red(), s);
            }
        }
        if let EmuError::ReadFail(_) = self {
            println!("{}", "Retrying..".yellow());
        } else {
            std::process::exit(1);
        }
    }
}
