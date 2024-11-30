use crate::Argument::*;
use crate::Instruction::*;
use crate::*;
impl CPU {
    pub fn execute_instruction(&mut self, ins: &Instruction) {
        match ins {
            HLT => self.running = false,
            ADD(arg1, arg2) => {
                let value = self.get_value(arg2);

                if let Register(n) = arg1 {
                    if *n == 6 {
                        let new_value = self.float_reg[0] + value;
                        if new_value.is_infinite() || new_value.is_nan() {
                            self.oflag = true;
                        }
                        self.float_reg[0] = new_value;
                    } else if *n == 7 {
                        let new_value = self.float_reg[1] + value;
                        if new_value.is_infinite() || new_value.is_nan() {
                            self.oflag = true;
                        }
                        self.float_reg[1] = new_value;
                    } else if *n > 5 {
                        UnrecoverableError::InvalidRegister(
                            self.pc,
                            Some("The register number is too large.".to_string()),
                        )
                        .err();
                    } else {
                        let current_value = self.int_reg[*n as usize];
                        let new_value = current_value as i32 + value as i32; // larger type for overflow
                        if new_value > i16::MAX as i32 || new_value < i16::MIN as i32 {
                            self.oflag = true;
                        }
                        self.int_reg[*n as usize] = new_value as i16;
                    }
                }
            }
            JGE(arg) => 'jge: {
                if self.int_reg[1] < self.int_reg[0] {
                    break 'jge;
                }
                if let MemAddr(n) = arg {
                    self.pc = *n as u16;
                    break 'jge;
                }
                if let SR(s) = arg {
                    self.jloc = self.pc;
                    self.pc = 20000 + (*s as u16 - 100);
                }
            }
            CL(arg) => {
                if let Flag(n) = arg {
                    match n {
                        0 => (),
                        1 => self.zflag = false,
                        2 => self.oflag = false,
                        _ => RecoverableError::UnknownFlag(
                            self.pc,
                            Some("Unknown flag in CL instruction".to_string()),
                        )
                        .err(),
                    }
                }
            }
            _ => print!(""),
        }
    }

    fn get_value(&mut self, arg: &Argument) -> f32 {
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
            RegPtr(n) => todo!(),
            _ => unreachable!(),
        }
    }

    fn handle_segmentation_fault(&mut self, message: &str) {
        UnrecoverableError::SegmentationFault(self.pc, Some(message.to_string())).err();
        if !CONFIG.quiet {
            println!("Attempting to recover by restarting...");
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        self.pc = self.starts_at;
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
