use crate::*;
use std::vec::Vec;
pub struct CPU {
    pub int_reg: [i16; 6],   // r0 thru r5
    pub float_reg: [f32; 2], // r6 and r7
    pub memory: [Option<i16>; 65536],
    pub pc: u16, // program counter
    pub ic: u16, // instruction counter
    pub sp: u16,
    pub bp: u16,
    pub jloc: u16, // location from which a jump was performed
    pub starts_at: u16,
    pub running: bool,
    pub zflag: bool,
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
            ic: 0,
            sp: 0,
            bp: 0,
            jloc: 0,
            starts_at: 0,
            running: false,
            zflag: false,
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
                    start_found = true;
                    self.shift_memory();
                    println!("program starts at {}", self.starts_at);
                }
                continue;
            }
            if (element >> 12) != 0b1111 {
                self.memory[counter] = Some(element);
                if CONFIG.verbose {
                    println!("Element {:016b} loaded into memory", element);
                }
            }
            counter += 1;
        }
    }
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
        if some_count + self.starts_at > 65535 {
            EmuError::MemoryOverflow().err();
        }
        let mem_copy = self.memory;
        self.memory = [None; 65536];
        for i in 0.=65536 {
            self.memory[(i + self.starts_at) as usize] = mem_copy[i as usize];
        }
        if CONFIG.verbose {
            println!("Shift completed.");
            println!("Memory: {:?}", self.memory);
        }
    }
}
// we need a function to load instructions into RAM
// we also need interrupts for pseudo-instructions
//
// debug messages would be nice too
