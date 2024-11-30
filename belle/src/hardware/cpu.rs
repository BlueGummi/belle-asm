use crate::Argument::*;
use crate::Instruction::*;
use crate::*;
use std::vec::Vec;
pub struct CPU {
    pub int_reg: [i16; 6],   // r0 thru r5
    pub float_reg: [f32; 2], // r6 and r7
    pub memory: [Option<i16>; 65536],
    pub pc: u16, // program counter
    pub sp: u16,
    pub bp: u16,
    pub ir: i16,
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
            pc: 1,
            sp: 0,
            bp: 0,
            ir: 0,
            jloc: 0,
            starts_at: 0,
            running: false,
            zflag: false,
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
            //    println!("Memory: {:?}", self.memory);
        }
    }
    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            // fetch instructions
            // execute instructions
            if self.memory[self.pc as usize].is_none() {
                if CONFIG.verbose {
                    println!("pc: {}", self.pc);
                }
                UnrecoverableError::SegmentationFault(
                    self.pc,
                    Some("Segmentation fault while finding next instruction".to_string()),
                )
                .err();
            }
            self.ir = self.memory[self.pc as usize].unwrap();
            let parsed_ins = self.parse_instruction();
            // self.execute_instruction(parsed_ins);
            self.pc += 1;
        }
    }
    fn parse_instruction(&self) -> Instruction {
        let opcode = (self.ir >> 12) & 0b0000000000001111u16 as i16;

        let ins_type = if ((self.ir >> 8) & 1) == 1 {
            1
        } else if ((self.ir >> 7) & 1) == 1 {
            2
        } else if ((self.ir >> 6) & 1) == 1 {
            3
        } else {
            0
        };
        let source = match ins_type {
            0 | 1 => self.ir & 0b11111111,
            2 => self.ir & 0b1111111,
            _ => self.ir & 0b111111,
        };
        let destination = (self.ir & 0b111000000000) >> 9;
        let part = match ins_type {
            0 => Register(source),
            1 => Literal(source),
            2 => MemPtr(source),
            _ => RegPtr(source),
        };
        let parsed_instruction = match opcode {
            HLT_OP => HLT,
            ADD_OP => ADD(Register(destination), part),
            JGE_OP => {
                if destination == 4 {
                    JGE(SR(source))
                } else {
                    JGE(MemAddr(source))
                }
            }
            CL_OP => CL(Flag(source)),
            DIV_OP => DIV(Register(destination), part),
            RET_OP => RET,
            LD_OP => LD(Nothing, Nothing),
            ST_OP => ST(Nothing, Nothing),
            SWP_OP => SWP(Nothing, Nothing),
            JZ_OP => JZ(Nothing),
            CMP_OP => CMP(Nothing, Nothing),
            MUL_OP => MUL(Nothing, Nothing),
            SET_OP => SET(Nothing),
            INT_OP => INT(Nothing),
            MOV_OP => MOV(Nothing, Nothing),
            _ => unreachable!(),
        };

        // println!("{:04b}", opcode);
        HLT
    }
}
// we need a function to load instructions into RAM
// we also need interrupts for pseudo-instructions
//
// debug messages would be nice too
