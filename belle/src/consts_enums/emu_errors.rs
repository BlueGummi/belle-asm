use crate::*;
use colored::*;
pub enum EmuError {
    FileNotFound(),
}
impl EmuError {
    pub fn err(&self) {
        print!("{} ", "Emulator Error:".red());
        match self {
            EmuError::FileNotFound() => {
                println!("File {} not found", CONFIG.file);
            }
        }
    }
}
