use crate::Argument::{Literal, MemAddr, MemPtr, RegPtr, Register};
use crate::Instruction::{
    ADD, CMP, DIV, HLT, INT, JGE, JZ, LD, MOV, MUL, NOP, POP, PUSH, RET, ST, SWP,
};
use crate::{
    Argument, Instruction, ADD_OP, CMP_OP, CPU, DIV_OP, HLT_OP, INT_OP, JGE_OP, JZ_OP, LD_OP,
    MOV_OP, MUL_OP, NOP_OP, POP_OP, PUSH_OP, RET_OP, ST_OP, SWP_OP,
};
use colored::*;
use std::arch::asm;
impl CPU {
    pub fn execute_instruction(&mut self, ins: &Instruction) {
        self.has_ran = true; // for debugger
        match ins {
            HLT => self.running = false,
            ADD(arg1, arg2) => self.handle_add(arg1, arg2),
            JGE(arg) => self.handle_jge(arg),
            POP(arg) => self.handle_pop(arg),
            DIV(arg1, arg2) => self.handle_div(arg1, arg2),
            RET => self.handle_ret(),
            LD(arg1, arg2) => self.handle_ld(arg1, arg2),
            ST(arg1, arg2) => self.handle_st(arg1, arg2),
            SWP(arg1, arg2) => self.handle_swp(arg1, arg2),
            JZ(arg) => self.handle_jz(arg),
            CMP(arg1, arg2) => self.handle_cmp(arg1, arg2),
            MUL(arg1, arg2) => self.handle_mul(arg1, arg2),
            PUSH(arg) => self.handle_push(arg),
            INT(arg) => self.handle_int(arg),
            MOV(arg1, arg2) => self.handle_mov(arg1, arg2),
            NOP => {
                // SAFETY: NOP
                unsafe {
                    asm!("nop");
                }
            } // NOP
              // _ => unreachable!(),
        }
        self.pc += 1;
    }

    pub fn get_register_value(&mut self, arg: &Argument) -> f32 {
        if let Register(n) = arg {
            match *n {
                6 => self.float_reg[0],
                7 => self.float_reg[1],
                n if n > 7 => {
                    self.report_invalid_register();
                    0.0 // default return value
                }
                _ => f32::from(self.int_reg[*n as usize]),
            }
        } else {
            0.0 // default return value if not a register
        }
    }

    pub fn set_register_value(&mut self, arg: &Argument, value: f32) {
        if let Register(n) = arg {
            match *n {
                6 => self.float_reg[0] = value,
                7 => self.float_reg[1] = value,
                n if n > 7 => self.report_invalid_register(),
                _ => self.int_reg[*n as usize] = value as i16,
            }
        }
    }

    pub fn get_value(&mut self, arg: &Argument) -> f32 {
        match arg {
            Register(n) => match n {
                4 => self.uint_reg[0].into(),
                5 => self.uint_reg[1].into(),
                6 => self.float_reg[0],
                7 => self.float_reg[1],
                _ => self.int_reg[*n as usize].into(),
            },
            Literal(n) => (*n).into(),
            MemPtr(n) => {
                if self.memory[*n as usize].is_none() {
                    self.handle_segmentation_fault(
                        "Segmentation fault while dereferencing pointer.\nThe pointer's location is empty.",
                    );
                }
                let tmp = self.memory[*n as usize].unwrap() as usize;
                if self.memory[tmp].is_none() {
                    self.handle_segmentation_fault(
                        "Segmentation fault while dereferencing pointer.\nThe address the pointer references is empty.",
                    );
                }
                self.memory[tmp].unwrap().into()
            }
            RegPtr(n) => {
                let tmp = match n {
                    4 => self.uint_reg[0].into(),
                    5 => self.uint_reg[1].into(),
                    6 => self.float_reg[0],
                    7 => self.float_reg[1],
                    _ => self.int_reg[*n as usize].into(),
                };
                let memloc: usize = tmp as usize;
                if self.memory[memloc].is_none() {
                    self.handle_segmentation_fault(
                        "Segmentation fault while dereferencing pointer.\nThe address the pointer references is empty.",
                    );
                    self.running = false;
                    return 0.0;
                }
                self.memory[memloc].unwrap().into()
            }
            MemAddr(n) => {
                if self.memory[*n as usize].is_none() {
                    self.handle_segmentation_fault(
                        "Segmentation fault while loading from memory.\nMemory address is empty.",
                    );
                }
                self.memory[*n as usize].unwrap().into()
            }
            _ => unreachable!("Argument types are invalid (how did you get here?)"),
        }
    }
    #[must_use]
    pub fn parse_instruction(&self) -> Instruction {
        let opcode = (self.ir >> 12) & 0b1111u16 as i16;
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
            1 => {
                if opcode == JZ_OP || opcode == JGE_OP {
                    self.ir & 0b111111111111
                } else {
                    let tmp = self.ir & 0b1111111;
                    if (self.ir & 0b10000000) >> 7 == 1 {
                        -tmp
                    } else {
                        tmp
                    }
                }
            }
            _ => {
                if opcode == JZ_OP || opcode == JGE_OP {
                    self.ir & 0b111111111111
                } else {
                    self.ir & 0b1111111
                }
            }
        };
        let destination = (self.ir & 0b111000000000) >> 9;
        let part = match ins_type {
            0 => Register(source),
            1 => Literal(source),
            2 => MemPtr(source),
            _ => RegPtr(source),
        };

        // println!("{:04b}", opcode);
        match opcode {
            HLT_OP => HLT,
            ADD_OP => ADD(Register(destination), part),
            JGE_OP => JGE(MemAddr(source)),
            POP_OP => POP(Register(source)),
            DIV_OP => DIV(Register(destination), part),
            RET_OP => RET,
            LD_OP => {
                let part = self.ir & 0b111111111;
                LD(Register(destination), MemAddr(part))
            }
            ST_OP => {
                let part = (self.ir & 0b111111111000) >> 3;
                ST(MemAddr(part), Register(self.ir & 0b111))
            }
            SWP_OP => SWP(Register(destination), Register(self.ir & 0b111)),
            JZ_OP => JZ(MemAddr(source)),
            CMP_OP => CMP(Register(destination), part),
            MUL_OP => MUL(Register(destination), part),
            PUSH_OP => PUSH(Register(source)),
            INT_OP => INT(Literal(source)),
            MOV_OP => MOV(Register(destination), part),
            NOP_OP => NOP,
            _ => {
                eprintln!(
                    "Cannot parse this. Code should be unreachable. {} line {}",
                    file!(),
                    line!()
                );
                MOV(Register(0), Register(0))
            }
        }
    }
}
