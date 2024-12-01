use belle::*;
use std::fs::File;
use std::io;
use std::path::Path;
use std::process;

fn main() -> io::Result<()> {
    if CONFIG.debug && CONFIG.verbose {
        EmuError::Impossible("Cannot have both debug and verbose flags".to_string()).err();
    }
    let executable_path = &CONFIG.file;
    if File::open(Path::new(executable_path)).is_err() {
        EmuError::FileNotFound().err();
        process::exit(1);
    }
    let bin = bin_to_vec(executable_path)?;
    if CONFIG.debug {
        run_bdb(executable_path)?;
    }
    if CONFIG.verbose {
        println!("CPU Initialized");
    }
    let mut cpu = CPU::new();
    cpu.load_binary(bin);
    cpu.run();
    Ok(())
}
