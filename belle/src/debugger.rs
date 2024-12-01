use crate::CPU;
use std::vec::Vec;
use std::fs::File;
use std::io::{self, Read, Write};
pub fn run_bdb(executable_path: &str) -> io::Result<()> {
    let prompt = "(bdb)> ";
    let mut dbgcpu = CPU::new();
    let mut clock = 0;
    let mut printed_bp = false;
    let mut breakpoints: Vec<u32> = Vec::new();

    loop {
        let bin = bin_to_vec(executable_path)?;
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let command = read_command()?;
        if command.is_empty() {
            continue;
        }

        let (cmd, arg) = parse_command(&command);
        match cmd.to_lowercase().as_str() {
            "q" | "quit" | ":q" => return exit_bdb(),
            "h" | "help" => handle_help(&arg),
            "l" | "load" => dbgcpu.load_binary(bin),
            "r" | "run" => handle_run(&mut dbgcpu),
            "s" | "step" => handle_step(&mut dbgcpu, &mut clock, &arg),
            "sb" => handle_backstep(&mut dbgcpu, &mut clock, &arg),
            "p" | "pmem" => handle_print_memory(&dbgcpu, &arg),
            "b" | "sbp" => handle_set_breakpoint(&mut breakpoints, &arg),
            "bp" => handle_print_breakpoints(&breakpoints),
            "br" => handle_remove_breakpoint(&mut breakpoints, &arg),
            "ba" => breakpoints.clear(),
            "i" | "info" => CPU::display_state(clock),
            _ => unknown_command(&command),
        }

        printed_bp = check_breakpoint(&breakpoints, clock, printed_bp);
    }
}

fn read_command() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn parse_command(command: &str) -> (&str, &str) {
    let mut parts = command.splitn(2, ' ');
    (parts.next().unwrap(), parts.next().unwrap_or(""))
}

fn exit_bdb() -> io::Result<()> {
    println!("Exiting...\n");
    Ok(())
}

fn handle_help(arg: &str) {
    if arg.is_empty() {
        println!("Available commands:");
        println!("q | quit | :q - Exit BDB");
        println!("h | help      - Print help on BDB or a specific command");
        println!("l | load      - Load program");
        println!("r | run       - Run loaded program");
        println!("s | step      - Step forward through the program once or a number of times");
        println!("sb            - Step back through the program once or a number of times");
        println!("p | pmem      - Print a value in memory (takes one argument)");
        println!("b | sbp       - Set a breakpoint at a specified memory address");
        println!("bp            - Print all breakpoints");
        println!("br            - Remove a specified breakpoint");
        println!("ba            - Remove all breakpoints");
        println!("i | info      - Print the current CPU state\n");
    } else {
        todo!("Help for specific command: {}", arg);
    }
}

fn handle_run(dbgcpu: &mut CPU) {
    if dbgcpu.memory.iter().all(|&x| x.is_none()) {
        eprintln!("CPU memory is empty.\nTry to load the program first.\n");
    } else {
        dbgcpu.run();
    }
}

fn handle_step(dbgcpu: &mut CPU, clock: &mut u32, arg: &str) {
    if !dbgcpu.has_ran {
        eprintln!("CPU has not run.\n");
        return;
    }
    if let Ok(n) = arg.parse::<u32>() {
        *clock += n;
    } else {
        *clock += 1;
    }
}

fn handle_backstep(dbgcpu: &mut CPU, clock: &mut u32, arg: &str) {
    if !dbgcpu.has_ran {
        eprintln!("CPU has not run.\n");
        return;
    }
    if let Ok(n) = arg.parse::<u32>() {
        if (*clock as i64) - (n as i64) < 0 {
            eprintln!("Stepping back not possible.\n");
        } else {
            *clock -= n;
        }
    } else {
        *clock -= 1;
    }
}

fn handle_print_memory(dbgcpu: &CPU, arg: &str) {
    if let Ok(n) = arg.parse::<usize>() {
        if let Some(memvalue) = dbgcpu.memory[n] {
            println!("Value in memory is:\n{:016b}", memvalue);
            println!("{}\n", memvalue);
        } else {
            println!("Nothing in memory here.\n");
        }
    } else {
        eprintln!("{} requires a numeric argument\n", "p | pmem");
    }
}

fn handle_set_breakpoint(breakpoints: &mut Vec<u32>, arg: &str) {
    if let Ok(n) = arg.parse::<u32>() {
        breakpoints.push(n);
        println!("Breakpoint set at {}\n", n);
    } else {
        eprintln!("Could not parse valid numeric value from {}\n", arg);
    }
}

fn handle_print_breakpoints(breakpoints: &[u32]) {
    for &breakpoint in breakpoints {
        println!("Breakpoint at {}", breakpoint);
    }
    println!();
}

fn handle_remove_breakpoint(breakpoints: &mut Vec<u32>, arg: &str) {
    if let Ok(n) = arg.parse::<u32>() {
        breakpoints.retain(|&x| x != n);
        println!("Breakpoints with value {} have been removed\n", n);
    } else {
        eprintln!("{} requires a numeric argument\n", "br");
    }
}

fn unknown_command(command: &str) {
    println!(
        "Unknown command: {}\nType help or h to view available commands\n",
        command
    );
}

fn check_breakpoint(breakpoints: &[u32], clock: u32, printed_bp: bool) -> bool {
    if breakpoints.contains(&clock) && !printed_bp {
        println!("Breakpoint reached.");
        CPU::display_state(clock);
        true
    } else {
        false
    }
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
