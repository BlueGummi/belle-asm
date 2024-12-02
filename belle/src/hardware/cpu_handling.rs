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
            self.pc = (SR_LOC as u16) + ((*s as u16) * 100 - 100);
        }
    }

    pub fn handle_cl(&mut self, arg: &Argument) {
        if let Flag(n) = arg {
            match n {
                0 => (),
                1 => self.zflag = false,
                2 => self.oflag = false,
                3 => self.hlt_on_overflow = false,
                4 => self.rflag = false,
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
                    if current_value as f32 % divisor != 0.0 {
                        self.rflag = true;
                    }
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
            self.pc = (SR_LOC as u16) + ((*s as u16) * 100 - 100);
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
                3 => self.hlt_on_overflow = true,
                4 => self.rflag = true,
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

    pub fn handle_int(&mut self, arg: &Argument) {
        let code = self.get_value(arg) as u16;
        match code {
            0_u16..=5_u16 => {
                println!("{}", self.int_reg[code as usize]);
            }
            6 => println!("{}", self.float_reg[0]),
            7 => println!("{}", self.float_reg[1]),
            8 => {
                let starting_point = self.int_reg[0];
                let end_point = self.int_reg[1];
                let memory = &self.memory;

                for index in starting_point..=end_point {
                    if let Some(value) = memory[index as usize] {
                        print!("{}", char::from_u32(value.try_into().unwrap()).unwrap());
                    } else {
                        println!();
                        self.handle_segmentation_fault(
                            "Segmentation fault. Memory index out of bounds on interrupt call.",
                        );
                        return;
                    }
                }
                println!();
            }
            9 => {
                let mut attempts = 0;
                let max_attempts = 10;
                
                while attempts < max_attempts {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    
                    match input.trim().parse::<i16>() {
                        Ok(number) => {
                            self.int_reg[0] = number;
                            break;
                        }
                        Err(e) => {
                            EmuError::ReadFail(e.to_string()).err();
                            attempts += 1;
                            if attempts == max_attempts {
                                println!("Failed to parse int from stdin 10 times, exiting...");
                                std::process::exit(1);
                            }
                        }
                    }
                }
            }
            10 => std::thread::sleep(std::time::Duration::from_secs(1)),
            11 => self.zflag = !self.zflag,
            12 => self.oflag = !self.oflag,
            13 => self.rflag = !self.rflag,
            14 => self.zflag = self.rflag,
            _ => todo!(),
        }
    }
}
