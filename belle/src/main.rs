use belle::*;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process;
use std::vec::Vec;
fn main() -> io::Result<()> {
    if CONFIG.verbose {
        println!("CPU Initialized");
    }
    let executable_path = &CONFIG.file;
    if File::open(Path::new(executable_path)).is_err() {
        EmuError::FileNotFound().err();
        process::exit(1);
    }
    let mut cpu = CPU::new();
    let bin = bin_to_vec(&executable_path)?;
    cpu.load_binary(bin);
    cpu.run();
    Ok(())
}

fn bin_to_vec(file_path: &str) -> io::Result<Vec<i16>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut result: Vec<i16> = Vec::new();
    for chunk in buffer.chunks(2) {
        if chunk.len() == 2 {
            let value = i16::from_be_bytes([chunk[0], chunk[1]]);
            result.push(value);
        }
    }
    Ok(result)
}
