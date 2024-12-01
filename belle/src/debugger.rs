use crate::CPU;
use crate::*;
use colored::*;
use std::fs::File;
use std::io::{self, Read, Write};
use std::vec::Vec;
fn cls() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}
pub fn run_bdb(executable_path: &str) -> io::Result<()> {
    let prompt = "(bdb)> ".green();
    let mut dbgcpu = CPU::new();
    let mut clock = 0;
    loop {
        let _ = ctrlc::set_handler(move || {
            println!("\nExiting...");
            std::process::exit(0);
        });
        let bin = bin_to_vec(executable_path)?;
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let command = input.trim().to_string();
        if command.is_empty() {
            continue;
        }
        let mut parts = command.splitn(2, ' ');

        let (cmd, arg) = (parts.next().unwrap(), parts.next().unwrap_or(""));
        match cmd.to_lowercase().as_str() {
            "q" | "quit" | ":q" => {
                println!("{}", "Exiting...\n".yellow());
                return Ok(());
            }
            "h" | "help" => {
                if arg.is_empty() {
                    println!("{}", "Available commands:".blue());
                    println!("q | quit | :q - Exit BDB");
                    println!("h | help      - Print help on BDB or a specific command");
                    println!("l | load      - Load program");
                    println!("r | run       - Run program");
                    println!("spc           - Set program counter");
                    println!("c | setclk    - Set clock");
                    println!("p | pmem      - Print value in memory");
                    println!("i | info      - Print CPU state at debugger's clock");
                    println!("wb            - Print CPU's starting memory address");
                    println!("cls           - Clear screen");
                    println!("e | exc       - Execute instruction");
                } else {
                    match arg.trim().to_lowercase().as_str() {
                        "q" | "quit" | ":q" => {
                            println!("'quit' takes no arguments.");
                            println!("'quit' exits the BELLE-debugger with code 0.");
                            println!(
                                "The CPU will stop execution or will exit if 'quit' is called.\n"
                            );
                        }
                        "h" | "help" => {
                            println!("'help' takes zero or one argument.");
                            println!("'help' prints out information on how to use commands.\n");
                        }
                        "l" | "load" => {
                            println!("'load' takes no arguments.");
                            println!("'load' loads the CPU's memory with the program, and does nothing afterwards.");
                            println!("'load' can be used to verify that the program is loaded correctly.");
                            println!(
                                "'p' | 'pmem' can be used after 'load' to view the CPU's memory\n"
                            );
                        }
                        "r" | "run" => {
                            println!("'run' takes no arguments");
                            println!(
                                "'run' executes the CPU with the data stored in its memory.\n"
                            );
                        }
                        "spc" => {
                            println!("'set program counter' takes one argument.");
                            println!("'spc' sets the CPU's program counter to a given number\n");
                        }
                        "c" | "clkset" => {
                            println!("'clock set' sets the CPU clock to a given value");
                            println!("If no value is set, the CPU clock is reset to 1\n");
                        }
                        "p" | "pmem" => {
                            println!("'print memory' takes one argument.");
                            println!("'pmem' prints the value at the specified memory address.");
                            println!("If nothing is there, it will say so.\n")
                        }
                        "e" | "exc" => {
                            println!("'execute' takes no arguments");
                            println!("'e' executes the instruction at the current memory address (program counter)\n")
                        }
                        "i" | "info" => {
                            println!("'info' takes no arguments.");
                            println!("'info' prints the current state of the CPU on the current clock cycle.\n");
                        }
                        "cls" | "clear" => {
                            println!("'clear' takes no arguments and resets the cursor to the top left\nof the terminal\n");
                        }
                        "wb" => {
                            println!("'where begins' takes no arugments");
                            println!("'wb' prints the starting memory address of the CPU\n")
                        }
                        _ => {
                            println!("Unknown command: '{}'", arg);
                            println!("Type 'h' or 'help' for a list of available commands.\n");
                        }
                    }
                }
            }
            "l" | "load" => dbgcpu.load_binary(bin),
            "r" | "run" => {
                if dbgcpu.memory.iter().all(|&x| x.is_none()) {
                    eprintln!(
                        "{}",
                        "CPU memory is empty.\nTry to load the program first.\n".red()
                    );
                } else {
                    dbgcpu.run();
                }
            }
            "spc" => 'spc: {
                if dbgcpu.memory.iter().all(|&x| x.is_none()) {
                    eprintln!(
                        "{}",
                        "CPU memory is empty.\nTry to load the program first.\n".red()
                    );
                    break 'spc;
                }
                if let Ok(n) = arg.parse::<u16>() {
                    dbgcpu.pc = n;
                    println!("Program counter set to {n}\n");
                } else {
                    eprintln!("{} requires a numeric argument\n", "p | pmem".red());
                }
            }
            "c" | "clkset" => {
                if !dbgcpu.has_ran {
                    eprintln!("{}", "CPU has not run.\n".red());
                    continue;
                }
                let n = arg.parse::<u32>().unwrap_or(1);
                clock = n;
            }
            "p" | "pmem" => {
                if let Ok(n) = arg.parse::<usize>() {
                    if let Some(memvalue) = dbgcpu.memory[n] {
                        println!("Value in memory is:\n{:016b}\n{}", memvalue, memvalue);
                    } else {
                        println!("{}", "Nothing in memory here.\n".yellow());
                    }
                } else {
                    eprintln!("{} requires a numeric argument\n", "p | pmem".red());
                }
            }
            "i" | "info" => CPU::display_state(clock),
            "wb" => {
                if dbgcpu.memory.iter().all(|&x| x.is_none()) {
                    eprintln!(
                        "{}",
                        "CPU memory is empty.\nTry to load the program first.\n".red()
                    );
                } else {
                    println!("Execution begins at memory address {}", dbgcpu.starts_at);
                }
            }
            "e" | "exc" => 'exc: {
                dbgcpu.ir = match dbgcpu.memory[dbgcpu.pc as usize] {
                    Some(value) => value,
                    None => {
                        eprintln!("Nothing at PC {}", dbgcpu.pc);
                        break 'exc;
                    }
                };
                let parsed_ins = dbgcpu.parse_instruction();
                dbgcpu.execute_instruction(&parsed_ins);
                dbgcpu.record_state();
                println!("  Integer Registers        : {:?}", dbgcpu.int_reg);
                println!("  Float Registers          : {:?}", dbgcpu.float_reg);
                println!("  Program Counter          : {}", dbgcpu.pc);
                println!("  Instruction Register     : {:016b}", dbgcpu.ir);
                println!("  Jump Location            : {}", dbgcpu.jloc);
                println!("  Running                  : {}", dbgcpu.running);
                println!("  Zero flag                : {}", dbgcpu.zflag);
                println!("  Overflow flag            : {}", dbgcpu.oflag);
                println!("  Remainder flag           : {}", dbgcpu.rflag);
                println!("  Disassembled Instruction : \n  {}\n", disassemble(dbgcpu.ir));
            }
            "cls" | "clear" => {
                cls();
            }
            _ => unknown_command(&command),
        }
    }
}

fn unknown_command(command: &str) {
    println!(
        "Unknown command: {}\nType help or h to view available commands\n",
        command.red()
    );
}

pub fn bin_to_vec(file_path: &str) -> io::Result<Vec<i16>> {
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
