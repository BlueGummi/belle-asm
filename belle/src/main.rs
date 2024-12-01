use belle::*;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;
use std::vec::Vec;
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
        let prompt = "(bdb)> ";
        let mut dbgcpu = CPU::new();
        let mut clock = 0;
        let mut printed_bp = false;
        let mut breakpoints: Vec<u32> = Vec::new();
        loop {
            let bin = bin_to_vec(executable_path)?;
            print!("{}", prompt);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let command = input.trim();
            if command.is_empty() {
                continue;
            }

            let mut parts = command.splitn(2, ' ');
            let cmd = parts.next().unwrap();
            let arg = parts.next().unwrap_or("");
            // let parsed_command: Command;
            match cmd.to_lowercase().as_str() {
                "q" | "quit" | ":q" => {
                    println!("Exiting...\n");
                    return Ok(());
                }
                "h" | "help" => {
                    if arg.is_empty() {
                        println!("Available commands:");
                        println!("q | quit | :q - Exit BDB");
                        println!("h | help      - Print help on BDB or a specific commmand");
                        println!("l | load      - Load program");
                        println!("r | run       - Run loaded program");
                        println!("s | step      - Step forward through the program once or a number of times");
                        println!("sb            - Step back through the program once or a number of times");
                        println!("p | pmem      - Print a value in memory (takes one argument");
                        println!("b | sbp       - Set a breakpoint at a specified memory address");
                        println!("bp            - Print all breakpoints");
                        println!("br            - Remove a specified breakpoint");
                        println!("ba            - Remove all breakpoints");
                        println!("i | info      - Print the current CPU state\n");
                    } else {
                        todo!();
                    }
                }
                "l" | "load" => dbgcpu.load_binary(bin),
                "r" | "run" => 'run: {
                    let all_none = dbgcpu.memory.iter().all(|&x| x.is_none());
                    if all_none {
                        eprintln!("CPU memory is empty.\nTry to load the program first.\n");
                        break 'run;
                    }
                    dbgcpu.run();
                }
                "s" | "step" => 'step: {
                    if !dbgcpu.has_ran {
                        eprintln!("CPU has not run.\n");
                        break 'step;
                    }
                    if !arg.is_empty() {
                        match arg.parse::<u32>() {
                            Ok(n) => clock += n,
                            Err(_) => {
                                eprintln!("Could not parse valid numeric value from {}\n", arg)
                            }
                        }
                    } else {
                        clock += 1;
                    }
                }
                "sb" => 'backstep: {
                    if !dbgcpu.has_ran {
                        eprintln!("CPU has not run.\n");
                        break 'backstep;
                    }
                    if !arg.is_empty() {
                        match arg.parse::<u32>() {
                            Ok(n) => {
                                if (clock as i64) - (n as i64) < 0 {
                                    eprintln!("Stepping back not possible.\n");
                                    break 'backstep;
                                }
                                clock -= n;
                            }
                            Err(_) => {
                                eprintln!("Could not parse valid numeric value from {}\n", arg)
                            }
                        }
                    } else {
                        if (clock as i64) - 1 < 0 {
                            eprintln!("Stepping back not possible.\n");
                            break 'backstep;
                        }
                        clock -= 1;
                    }
                }
                "p" | "pmem" => {
                    if !arg.is_empty() {
                        match arg.parse::<usize>() {
                            Ok(n) => {
                                let memvalue = dbgcpu.memory[n];
                                if let Some(n) = memvalue {
                                    println!("Value in memory is:\n{:016b}", n);
                                    println!("{}\n", n);
                                } else {
                                    println!("Nothing in memory here.\n");
                                }
                            }
                            Err(_) => {
                                eprintln!("Could not parse valid numeric value from {}\n", arg)
                            }
                        }
                    } else {
                        eprintln!("{} requires a numeric argument\n", cmd);
                    }
                }
                "b" | "sbp" => {
                    if !arg.is_empty() {
                        match arg.parse::<u32>() {
                            Ok(n) => {
                                breakpoints.push(n);
                                println!("Breakpoint set at {}\n", n);
                            }
                            Err(_) => {
                                eprintln!("Could not parse valid numeric value from {}\n", arg)
                            }
                        }
                    } else {
                        eprintln!("{} requires a numeric argument\n", cmd);
                    }
                }
                "bp" => {
                    for breakpoint in &breakpoints {
                        println!("Breakpoint at {breakpoint}");
                    }
                    println!();
                }
                "br" => {
                    if !arg.is_empty() {
                        match arg.parse::<u32>() {
                            Ok(n) => {
                                breakpoints.retain(|&x| x != n);
                                println!("Breakpoints with value {} have been removed\n", n);
                            }
                            Err(_) => {
                                eprintln!("Could not parse valid numeric value from {}\n", arg)
                            }
                        }
                    } else {
                        eprintln!("{} requires a numeric argument\n", cmd);
                    }
                }
                "ba" => breakpoints.clear(),
                "i" | "info" => {
                    belle::CPU::display_state(clock);
                }
                _ => println!(
                    "Unknown command: {}\nType help or h to view available commands\n",
                    command
                ),
            }
            if breakpoints.contains(&clock) && !printed_bp {
                println!("Breakpoint reached.");
                belle::CPU::display_state(clock);
                printed_bp = true;
            } else {
                printed_bp = false; // so we don't print it twice
            }
            // println!("Command: {}, Argument: {}", cmd, arg);
        }
    }
    if CONFIG.verbose {
        println!("CPU Initialized");
    }
    let mut cpu = CPU::new();
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
