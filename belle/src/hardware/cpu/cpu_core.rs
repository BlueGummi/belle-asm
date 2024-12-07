use crate::Argument::*;
use crate::Instruction::*;
use crate::{CLOCK, CONFIG, EmuError, RecoverableError, UnrecoverableError, cpu};
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
    pub ir: i16,                                 // location from which a jump was performed
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
    #[must_use] pub fn new() -> CPU {
        CPU {
            int_reg: [0; 6],
            float_reg: [0.0; 2],
            memory: Box::new([None; MEMORY_SIZE]),
            pc: 0,
            ir: 0,
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
                // start directive
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
                println!("Element {element:016b} loaded into memory");
            }

            counter += 1;
        }
        self.shift_memory();
        self.pc = self.starts_at;
    }

    fn shift_memory(&mut self) {
        if let Some(first_val) = self.memory.iter().position(|&e| e.is_some()) {
            if self.pc == first_val as u16 {
                return;
            }
        }

        if CONFIG.verbose {
            println!("Shifting memory...");
        }

        let some_count = self.memory.iter().filter(|&&e| e.is_some()).count();

        if some_count as u32 + u32::from(self.starts_at) > MEMORY_SIZE.try_into().unwrap() {
            EmuError::MemoryOverflow().err();
        }

        let mut new_memory = Box::new([None; MEMORY_SIZE]);

        let first_some_index = self.memory.iter().position(|&e| e.is_some()).unwrap_or(0);
        for (i, value) in self.memory.iter().enumerate() {
            if let Some(val) = value {
                let new_index = (self.starts_at + (i - first_some_index) as u16) as usize;
                new_memory[new_index] = Some(*val);
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
        let mut restart_count = 0;
        while self.running {
            let _ = ctrlc::set_handler(move || {
                println!("Halting...");
                std::process::exit(0);
            });
            let mut clock = CLOCK.lock().unwrap(); // might panic
            *clock += 1;
            std::thread::sleep(std::time::Duration::from_millis(
                CONFIG.time_delay.unwrap().into(),
            ));
            std::mem::drop(clock); // clock must go bye bye so it unlocks
            if self.memory[self.pc as usize].is_none() {
                if CONFIG.verbose {
                    println!("PC: {}", self.pc);
                }
                UnrecoverableError::SegmentationFault(
                    self.pc,
                    Some("Segmentation fault while finding next instruction".to_string()),
                )
                .err();
                if restart_count > 10 {
                    println!("More than ten restarts have been attempted.\nExiting...");
                    return;
                }
                if !CONFIG.quiet {
                    println!("Attempting to recover by restarting...");
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
                self.pc = self.starts_at;
                restart_count += 1;
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
