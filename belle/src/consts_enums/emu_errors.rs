use crate::*;
use colored::*;
pub enum EmuError {
    FileNotFound(),
    MemoryOverflow(),
    Duplicate(String),
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
        }
        std::process::exit(1);
    }
}
