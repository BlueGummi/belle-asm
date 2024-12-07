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
        if self.sflag || !self.zflag {
            return;
        }
        if let MemAddr(n) = arg {
            self.handle_push(&Argument::Literal(self.pc.try_into().unwrap()));
            self.pc = (*n as u16) - 2;
        }
    }

    pub fn handle_pop(&mut self, arg: &Argument) {
        if let Register(_) = arg {
            let temp: i32 = if self.sp >= self.bp {
                (self.sp - 1).into()
            } else {
                (self.sp + 1).into()
            };
            match self.memory[temp as usize] {
                Some(v) => {
                    self.set_register_value(arg, v.into());
                    if self.sp > self.bp {
                        self.memory[(self.sp - 1) as usize] = None;
                        self.sp -= 1;
                    } else {
                        self.memory[(self.sp + 1) as usize] = None;
                        self.sp += 1;
                    }
                }
                None => {
                    UnrecoverableError::SegmentationFault(
                        self.pc,
                        Some("Segmentation fault whilst POPping off the stack".to_string()),
                    )
                    .err();
                    if !CONFIG.debug {
                        std::process::exit(1);
                    }
                }
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
        let temp: i32 = if self.sp >= self.bp {
            (self.sp - 1).into()
        } else {
            (self.sp + 1).into()
        };
        match self.memory[temp as usize] {
            Some(v) => {
                self.pc = v as u16;
                if self.sp > self.bp {
                    self.memory[(self.sp - 1) as usize] = None;
                    self.sp -= 1;
                } else {
                    self.memory[(self.sp + 1) as usize] = None;
                    self.sp += 1;
                }
            }
            None => {
                self.handle_ret_error("Cannot return.");
                if !CONFIG.debug {
                    std::process::exit(1);
                }
            }
        }
    }

    fn handle_ret_error(&mut self, message: &str) {
        UnrecoverableError::SegmentationFault(self.pc, Some(message.to_string())).err();
        if !CONFIG.debug {
            std::process::exit(1);
        }
        println!("Resetting...");
        self.pc = self.starts_at;
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
            self.handle_push(&Argument::Literal(self.pc.try_into().unwrap()));
            self.pc = (*n as u16) - 2;
        }
    }

    pub fn handle_cmp(&mut self, arg1: &Argument, arg2: &Argument) {
        let src = self.get_value(arg2);
        if let Register(_) = arg1 {
            let value = self.get_register_value(arg1);
            let result = value - src;
            self.zflag = (result).abs() < f32::EPSILON;
            self.sflag = result < 0.0;
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

    pub fn handle_push(&mut self, arg: &Argument) {
        let mut val: f32 = 0.0;
        if let Literal(l) = arg {
            val = (*l).into();
        }

        if let Register(_) = arg {
            val = self.get_register_value(arg);
        }
        if self.sp >= self.bp {
            for i in self.bp..=self.sp {
                if self.memory[i as usize].is_none() {
                    self.memory[i as usize] = Some(val as i16);
                    self.sp = i + 1;
                    break;
                }
            }
        } else {
            let mut i = self.bp;
            while i >= self.sp {
                if self.memory[i as usize].is_none() {
                    self.memory[i as usize] = Some(val as i16);
                    self.sp = i - 1; // set the stack pointer
                    break;
                }
                i -= 1; // down by one
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
                            "Segmentation fault. Memory index out of bounds on interrupt call 8.",
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
            11 => self.zflag = true,
            12 => self.zflag = false,
            13 => self.zflag = !self.zflag,

            21 => self.oflag = true,
            22 => self.oflag = false,
            23 => self.oflag = !self.oflag,

            31 => self.rflag = true,
            32 => self.rflag = false,
            33 => self.rflag = !self.rflag,

            41 => self.sflag = true,
            42 => self.sflag = false,
            43 => self.sflag = !self.sflag,

            51 => self.hlt_on_overflow = true,
            52 => self.hlt_on_overflow = false,
            53 => self.hlt_on_overflow = !self.hlt_on_overflow,

            // 10 - 20 set flags
            // 20 - 30 unset them
            // 30 - 40 invert them
            _ => todo!(),
        }
    }
}