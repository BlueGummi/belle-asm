use crate::Argument::*;
use crate::Instruction::*;
use crate::*;
use colored::*;
use std::vec::Vec;
#[derive(Clone)]
pub struct CPU {
    pub int_reg: [i16; 6],   // r0 thru r5
    pub float_reg: [f32; 2], // r6 and r7
    pub memory: [Option<i16>; 65536],
    pub pc: u16, // program counter
    pub ir: i16,
    pub jloc: u16, // location from which a jump was performed
    pub starts_at: u16,
    pub running: bool,
    pub zflag: bool,
    pub oflag: bool,
}
impl Default for CPU {
    fn default() -> CPU {
        CPU::new()
    }
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            int_reg: [0; 6],
            float_reg: [0.0; 2],
            memory: [None; 65536],
            pc: 0,
            ir: 0,
            jloc: 0,
            starts_at: 0,
            running: false,
            zflag: false,
            oflag: false,
        }
    }
    pub fn load_binary(&mut self, binary: Vec<i16>) {
        let mut in_subr = false;
        let mut counter = 0;
        let mut start_found = false;
        let mut subr_loc = 20000;
        for element in binary {
            if in_subr && (element >> 12) != RET_OP {
                self.memory[counter + subr_loc] = Some(element);
                continue;
            } else if (element >> 12) == RET_OP {
                in_subr = false;
                self.memory[counter + subr_loc] = Some(element);
                subr_loc += 100;
            }
            if (element >> 9) == 1 {
                if start_found {
                    EmuError::Duplicate(".start directives".to_string()).err();
                }
                self.starts_at = (element & 0b111111111) as u16;
                if CONFIG.verbose {
                    println!(".start directive found.");
                }
                start_found = true;
                self.shift_memory();
                if CONFIG.verbose {
                    println!("program starts at {}", self.starts_at);
                }
                continue;
            }
            if (element >> 12) & 0b0000000000001111u16 as i16 != 0b1111 {
                self.memory[counter + self.starts_at as usize] = Some(element);
                if CONFIG.verbose {
                    println!("Element {:016b} loaded into memory", element);
                }
            } else {
                in_subr = true;
                continue;
            }
            counter += 1;
        }
    }
    #[allow(unused_comparisons)]
    fn shift_memory(&mut self) {
        let mut some_count = 0;
        if CONFIG.verbose {
            println!("Shifting memory...");
        }
        for element in self.memory {
            if element.is_some() {
                some_count += 1;
            }
        }
        // check for overflow
        if some_count as u32 + self.starts_at as u32 > 65535 {
            EmuError::MemoryOverflow().err();
        }
        let mem_copy = self.memory;
        self.memory = [None; 65536];
        for i in 0..=65535 {
            if mem_copy[i as usize].is_some() {
                self.memory[(i + self.starts_at) as usize] = mem_copy[i as usize];
            }
        }
        self.pc = self.starts_at;
        if CONFIG.verbose {
            println!("Shift completed.");
            println!("Memory: {:?}", self.memory);
        }
    }
    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            let mut clock = CLOCK.lock().unwrap(); // might panic
            *clock += 1;
            std::mem::drop(clock); // clock must go bye bye so it unlocks
            if self.memory[self.pc as usize].is_none() {
                if CONFIG.verbose {
                    println!("pc: {}", self.pc);
                }
                UnrecoverableError::SegmentationFault(
                    self.pc,
                    Some("Segmentation fault while finding next instruction".to_string()),
                )
                .err();
                if !CONFIG.quiet {
                    println!("Attempting to recover by restarting...")
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
                self.pc = self.starts_at;
            }
            self.ir = self.memory[self.pc as usize].unwrap();
            let parsed_ins = self.parse_instruction();
            self.execute_instruction(&parsed_ins);
            self.pc += 1;
            self.record_state();
            let clock = CLOCK.lock().unwrap();
            cpu::CPU::display_state(*clock);
        }
        if !self.running {
            if !CONFIG.quiet {
                println!("Halting...");
            }
            let mut clock = CLOCK.lock().unwrap(); // might panic
            *clock += 1;
            std::mem::drop(clock);
            self.record_state();
            let clock = CLOCK.lock().unwrap(); // might panic
            cpu::CPU::display_state(*clock);
            std::process::exit(0);
        }
    }
}
// we need a function to load instructions into RAM
// we also need interrupts for pseudo-instructions
//
// debug messages would be nice too
