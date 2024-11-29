use belle::*;
use std::fs::File;
use std::path::Path;
use std::process;
fn main() {
    if CONFIG.verbose {
        println!("CPU Initialized");
    }
    let executable = &CONFIG.file;
    if File::open(Path::new(executable)).is_err() {
        EmuError::FileNotFound().err();
        process::exit(1);
    }
    let mut cpu = CPU::new();
}
