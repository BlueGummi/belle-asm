use crate::Argument::*;
use crate::Instruction::*;
use crate::*;
impl CPU {
    pub fn handle_add(&mut self, arg1: &Argument, arg2: &Argument) {
        let value = self.get_value(arg2);
        if let Register(n) = arg1 {
            match *n {
                6 => self.float_reg[0] += value,
                7 => self.float_reg[1] += value,
                n if n > 5 => self.report_invalid_register(),
                _ => {
                    let current_value = self.int_reg[*n as usize];
                    let new_value = current_value as i32 + value as i32;
                    self.check_overflow(new_value);
                    self.int_reg[*n as usize] = new_value as i16;
                }
            }
        }
    }

    pub fn handle_jge(&mut self, arg: &Argument) {
        if self.int_reg[1] < self.int_reg[0] {
            return;
        }
        if let MemAddr(n) = arg {
            self.pc = *n as u16;
        } else if let SR(s) = arg {
            self.jloc = self.pc;
            self.pc = 20000 + ((*s as u16)*100 - 100);
        }
    }

    pub fn handle_cl(&mut self, arg: &Argument) {
        if let Flag(n) = arg {
            match n {
                0 => (),
                1 => self.zflag = false,
                2 => self.oflag = false,
                _ => self.report_unknown_flag("CL"),
            }
        }
    }

    pub fn handle_div(&mut self, arg1: &Argument, arg2: &Argument) {
        let divisor = self.get_value(arg2);
        if divisor == 0.0 {
            self.report_divide_by_zero();
            self.pc = self.starts_at;
            return;
        }
        if let Register(n) = arg1 {
            match *n {
                6 => self.float_reg[0] /= divisor,
                7 => self.float_reg[1] /= divisor,
                n if n > 5 => self.report_invalid_register(),
                _ => {
                    let current_value = self.int_reg[*n as usize];
                    self.int_reg[*n as usize] = (current_value as i32 / divisor as i32) as i16;
                }
            }
        }
    }

    pub fn handle_ret(&mut self) {
        self.pc = self.jloc;
    }

    pub fn handle_ld(&mut self, arg1: &Argument, arg2: &Argument) {
        let source = self.get_value(arg2);
        if let Register(n) = arg1 {
            match *n {
                6 => self.float_reg[0] = source,
                7 => self.float_reg[1] = source,
                n if n > 5 => self.report_invalid_register(),
                _ => {
                    self.check_overflow(source as i32);
                    self.int_reg[*n as usize] = source as i16;
                }
            }
        }
    }

    pub fn handle_st(&mut self, arg1: &Argument, arg2: &Argument) {
        let source = self.get_value(arg2);
        if let MemAddr(n) = arg1 {
            self.memory[*n as usize] = Some(source as i16);
        }
    }

    pub fn handle_swp(&mut self, arg1: &Argument, arg2: &Argument) {
        let source = self.get_register_value(arg2);
        if let Register(_) = arg1 {
            let dest = self.get_register_value(arg1);
            self.set_register_value(arg1, source);
            self.set_register_value(arg2, dest);
        }
    }

    pub fn handle_jz(&mut self, arg: &Argument) {
        if !self.zflag {
            return;
        }
        if let MemAddr(n) = arg {
            self.pc = *n as u16;
        } else if let SR(s) = arg {
            self.jloc = self.pc;
            self.pc = 20000 + ((*s as u16)*100 - 100);
        }
    }

    pub fn handle_cmp(&mut self, arg1: &Argument, arg2: &Argument) {
        let src = self.get_value(arg2);
        if let Register(_) = arg1 {
            let value = self.get_register_value(arg1);
            self.zflag = (value - src).abs() < f32::EPSILON;
        }
    }

    pub fn handle_mul(&mut self, arg1: &Argument, arg2: &Argument) {
        let value = self.get_value(arg2);
        if let Register(n) = arg1 {
            match *n {
                6 => self.float_reg[0] *= value,
                7 => self.float_reg[1] *= value,
                n if n > 5 => self.report_invalid_register(),
                _ => {
                    let current_value = self.int_reg[*n as usize];
                    let new_value = current_value as i32 * value as i32;
                    self.check_overflow(new_value);
                    self.int_reg[*n as usize] = new_value as i16;
                }
            }
        }
    }

    pub fn handle_set(&mut self, arg: &Argument) {
        if let Flag(n) = arg {
            match n {
                0 => (),
                1 => self.zflag = true,
                2 => self.oflag = true,
                _ => self.report_unknown_flag("SET"),
            }
        }
    }
    pub fn handle_mov(&mut self, arg1: &Argument, arg2: &Argument) {
        let value = self.get_value(arg2);
        if let Register(n) = arg1 {
            match *n {
                6 => self.float_reg[0] = value,
                7 => self.float_reg[1] = value,
                n if n > 5 => self.report_invalid_register(),
                _ => {
                    self.int_reg[*n as usize] = value as i16;
                }
            }
        }
    }
}
