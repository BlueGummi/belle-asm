use crate::Argument::*;
use crate::Instruction::*;
use crate::*;
use colored::*;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

pub const MEMORY_SIZE: usize = 512;

#[derive(Clone)]
pub struct CPU {
    pub int_reg: [i16; 6],                       // r0 thru r5
    pub float_reg: [f32; 2],                     // r6 and r7
    pub memory: Box<[Option<i16>; MEMORY_SIZE]>, // Use Box to allocate the array on the heap
    pub pc: u16,                                 // program counter
    pub ir: i16,
    pub jlocs: Vec<u16>, // location from which a jump was performed
    pub starts_at: u16,
    pub running: bool,
    pub has_ran: bool,
    pub zflag: bool,
    pub oflag: bool,
    pub rflag: bool,
    pub sflag: bool,
    pub hlt_on_overflow: bool,
    pub sp: u16,
    pub bp: u16,
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
            memory: Box::new([None; MEMORY_SIZE]),
            pc: 0,
            ir: 0,
            jlocs: Vec::new(),
            starts_at: 100,
            running: false,
            has_ran: false,
            zflag: false,
            oflag: false,
            rflag: false,
            sflag: false,
            hlt_on_overflow: false,
            sp: 0,
            bp: 100,
        }
    }

    pub fn load_binary(&mut self, binary: Vec<i16>) {
        let mut counter = 0;
        let mut start_found = false;

        for element in binary {
            if (element >> 9) == 1 {
                if start_found {
                    EmuError::Duplicate(".start directives".to_string()).err();
                }
                self.starts_at = (element & 0b111111111) as u16;
                if CONFIG.verbose {
                    println!(".start directive found.");
                }
                start_found = true;
                if CONFIG.verbose {
                    println!("program starts at {}", self.starts_at);
                }
                continue;
            } else if (element >> 9) == 2 {
                self.sp = (element & 0b111111111) as u16;
                if CONFIG.verbose {
                    println!(".ssp directive found");
                }
                continue;
            } else if (element >> 9) == 3 {
                self.bp = (element & 0b111111111) as u16;
                if CONFIG.verbose {
                    println!(".sbp directive found");
                }
                continue;
            }
                self.memory[counter + self.starts_at as usize] = Some(element);
                if CONFIG.verbose {
                    println!("Element {:016b} loaded into memory", element);
                }
            
            counter += 1;
        }
        self.pc = self.starts_at;
        // self.shift_memory();
    }

    #[allow(unused_comparisons)]
    #[allow(dead_code)]
    fn shift_memory(&mut self) {
        let mut first_val = usize::MAX;
        for (i, element) in self.memory.iter().enumerate() {
            if element.is_some() {
                first_val = i;
                break;
            }
        }

        if first_val != usize::MAX && self.pc == first_val as u16 {
            return;
        }

        let mut some_count = 0;
        if CONFIG.verbose {
            println!("Shifting memory...");
        }

        for element in self.memory.iter() {
            if element.is_some() {
                some_count += 1;
            }
        }

        if some_count as u32 + self.starts_at as u32 > MEMORY_SIZE.try_into().unwrap() {
            EmuError::MemoryOverflow().err();
        }

        let mut new_memory = Box::new([None; MEMORY_SIZE]);

        for i in 0..MEMORY_SIZE {
            if let Some(value) = self.memory[i].take() {
                new_memory[(i as u16 + self.starts_at) as usize] = Some(value);
            }
        }
        std::mem::swap(&mut self.memory, &mut new_memory);
        self.pc = self.starts_at;

        if CONFIG.verbose {
            println!("Shift completed.");
        }
    }

    pub fn run(&mut self) {
        self.has_ran = true; // for debugger
        self.running = true;
        if CONFIG.verbose {
            println!("  Starts At MemAddr: {}", self.starts_at);
        }
        while self.running {
            let mut clock = CLOCK.lock().unwrap(); // might panic
            *clock += 1;
            std::thread::sleep(std::time::Duration::from_millis(
                CONFIG.time_delay.unwrap().into(),
            ));
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
            if CONFIG.debug || CONFIG.verbose {
                self.record_state();
            }
            let clock = CLOCK.lock().unwrap();
            if CONFIG.verbose {
                cpu::CPU::display_state(*clock);
            }
            if self.oflag {
                RecoverableError::Overflow(self.pc, Some("Overflowed a register.".to_string()))
                    .err();
                if self.hlt_on_overflow {
                    self.running = false;
                }
            }
        }
        if !self.running {
            if CONFIG.verbose {
                println!("Halting...");
            }
            let mut clock = CLOCK.lock().unwrap(); // might panic
            *clock += 1;
            std::mem::drop(clock);
            self.record_state();
            let clock = CLOCK.lock().unwrap(); // might panic
            if CONFIG.verbose {
                cpu::CPU::display_state(*clock);
            }
            if !CONFIG.debug {
                std::process::exit(0);
            }
        }
    }
}
