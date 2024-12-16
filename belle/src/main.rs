/*
 * Copyright (c) 2024 BlueGummi
 * All rights reserved.
 *
 * This code is licensed under the BSD 3-Clause License.
 */
use belle::{bin_to_vec, run_bdb, EmuError, CONFIG, CPU};
use std::fs::{self, File};
use std::io;
use std::path::Path;
use std::process;

fn main() -> io::Result<()> {
    if CONFIG.debug && CONFIG.verbose {
        eprintln!(
            "{}",
            EmuError::Impossible("Cannot have both debug and verbose flags".to_string())
        );
        process::exit(1);
    }
    if CONFIG.quiet && CONFIG.verbose {
        eprintln!(
            "{}",
            EmuError::Impossible("Cannot have both debug and quiet flags".to_string())
        );
        process::exit(1);
    }
    let executable_path = &CONFIG.file;

    if let Ok(metadata) = fs::metadata(executable_path) {
        if metadata.is_dir() {
            eprintln!("{}", EmuError::IsDirectory());
            process::exit(1);
        }
    }
    if File::open(Path::new(executable_path)).is_err() {
        eprintln!("{}", EmuError::FileNotFound());
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
    cpu.load_binary(&bin);
    if let Err(e) = cpu.run() {
        eprintln!("{e}");
        process::exit(1);
    }
    Ok(())
}
