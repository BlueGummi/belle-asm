use crate::Argument::*;
use crate::Instruction::*;
use crate::*;
impl CPU {
    pub fn execute_instruction(&mut self, ins: &Instruction) {
        match ins {
            HLT => self.running = false,
            ADD(arg1, arg2) => self.handle_add(arg1, arg2),
            JGE(arg) => self.handle_jge(arg),
            CL(arg) => self.handle_cl(arg),
            DIV(arg1, arg2) => self.handle_div(arg1, arg2),
            RET => self.handle_ret(),
            LD(arg1, arg2) => self.handle_ld(arg1, arg2),
            ST(arg1, arg2) => self.handle_st(arg1, arg2),
            SWP(arg1, arg2) => self.handle_swp(arg1, arg2),
            JZ(arg) => self.handle_jz(arg),
            CMP(arg1, arg2) => self.handle_cmp(arg1, arg2),
            MUL(arg1, arg2) => self.handle_mul(arg1, arg2),
            SET(arg) => self.handle_set(arg),
            //INT(arg) => self.handle_int(arg),
            MOV(arg1, arg2) => self.handle_mov(arg1, arg2),
            _ => print!(""),
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
                _ => self.int_reg[*n as usize] as f32,
            }
        } else {
            0.0 // default return value if not a Register
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
            Register(n) => {
                if *n == 6 {
                    self.float_reg[0]
                } else if *n == 7 {
                    self.float_reg[1]
                } else {
                    self.int_reg[*n as usize].into()
                }
            }
            Literal(n) => (*n).into(),
            MemPtr(n) => {
                if self.memory[*n as usize].is_none() {
                    self.handle_segmentation_fault(
                        "Segmentation fault while dereferencing pointer. 
                    The pointer's location is empty.",
                    );
                }
                let tmp = self.memory[*n as usize].unwrap() as usize;
                if self.memory[tmp].is_none() {
                    self.handle_segmentation_fault(
                        "Segmentation fault while dereferencing pointer. 
                    The address the pointer references is empty.",
                    );
                }
                self.memory[tmp].unwrap().into()
            }
            RegPtr(n) => {
                let tmp = if *n == 6 {
                    self.float_reg[0]
                } else if *n == 7 {
                    self.float_reg[1]
                } else {
                    self.int_reg[*n as usize].into()
                };
                let memloc: usize = tmp as usize;
                if self.memory[memloc].is_none() {
                    self.handle_segmentation_fault(
                        "Segmentation fault while dereferencing pointer. 
                    The address the pointer references is empty.",
                    );
                }
                self.memory[memloc].unwrap().into()
            }
            MemAddr(n) => {
                if self.memory[*n as usize].is_none() {
                    self.handle_segmentation_fault(
                        "Segmentation fault while loading from memory.
                        Memory address is empty.",
                    );
                }
                self.memory[*n as usize].unwrap().into()
            }
            _ => unreachable!(),
        }
    }
    pub fn parse_instruction(&self) -> Instruction {
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
            0 => self.ir & 0b111,
            1 => {
                let tmp = self.ir & 0b1111111;
                if (self.ir & 0b10000000) >> 7 == 1 {
                    -tmp
                } else {
                    tmp
                }
            }
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

        // println!("{:04b}", opcode);
        match opcode {
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
            LD_OP => {
                let part = self.ir & 0b111111111;
                LD(Register(destination), MemAddr(part))
            }
            ST_OP => {
                let part = (self.ir & 0b111111111000) >> 3;
                ST(MemAddr(part), Register(self.ir & 0b111))
            }
            SWP_OP => SWP(Register(destination), Register(self.ir & 0b111)),
            JZ_OP => {
                if destination == 4 {
                    JZ(SR(source))
                } else {
                    JZ(MemAddr(source))
                }
            }
            CMP_OP => CMP(Register(destination), part),
            MUL_OP => MUL(Register(destination), part),
            SET_OP => SET(Flag(source)),
            INT_OP => INT(Literal(source)),
            MOV_OP => MOV(Register(destination), part),
            _ => unreachable!(),
        }
    }
}
