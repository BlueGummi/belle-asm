use crate::Argument::*;
use crate::*;
use std::io::{self, Read, Write};
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

impl CPU {
    pub fn handle_add(&mut self, arg1: &Argument, arg2: &Argument) {
        let value = self.get_value(arg2);
        if let Register(n) = arg1 {
            let new_value = match *n {
                4 => {
                    self.uint_reg[0] += value as u16;
                    self.uint_reg[0] as i64 + value as i64
                }
                5 => {
                    self.uint_reg[1] += value as u16;
                    self.uint_reg[1] as i64 + value as i64
                }
                6 => {
                    self.float_reg[0] += value;
                    self.float_reg[0] as i64 + value as i64
                }
                7 => {
                    self.float_reg[1] += value;
                    self.float_reg[1] as i64 + value as i64
                }
                n if n > 5 => {
                    self.report_invalid_register();
                    0
                }
                _ => {
                    self.int_reg[*n as usize] += value as i16;
                    self.int_reg[*n as usize] as i64 + value as i64
                }
            };
            self.check_overflow(new_value, *n as u16);
        }
    }

    pub fn handle_jo(&mut self, arg: &Argument) {
        if !self.oflag {
            return;
        }
        self.handle_push(&Argument::Literal(self.pc.try_into().unwrap()));
        if let MemAddr(n) = arg {
            self.pc = (*n as u16) - 2;
        } else if let RegPtr(n) = arg {
            self.pc = self.get_value(&Argument::Register(*n)) as u16;
        }
    }

    pub fn handle_pop(&mut self, arg: &Argument) {
        if let Register(_) = arg {
            /*let temp: i32 = if self.sp >= self.bp {
                (self.sp - 1).into()
            } else {
                (self.sp + 1).into()
            };
            */
            let temp: i32 = self.sp.into();
            if let Some(v) = self.memory[temp as usize] {
                self.set_register_value(arg, v.into());
                if self.sp > self.bp {
                    if self.sp != self.bp {
                        RecoverableError::BackwardStack(self.pc, None).err();
                    }
                    self.memory[self.sp as usize] = None;
                    if self.sp != self.bp {
                        self.sp -= 1;
                    }
                } else {
                    self.memory[self.sp as usize] = None;
                    if self.sp != self.bp {
                        self.sp += 1;
                    }
                }
            } else {
                UnrecoverableError::StackUnderflow(self.pc, None).err();
                self.running = false;
                if !CONFIG.debug {
                    std::process::exit(1);
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
            let new_value = match *n {
                4 => {
                    if self.uint_reg[0] as f32 % divisor != 0.0 {
                        self.rflag = true;
                    }
                    self.uint_reg[0] /= divisor as u16;
                    self.uint_reg[0] as i64 / divisor as i64
                }
                5 => {
                    if self.uint_reg[1] as f32 % divisor != 0.0 {
                        self.rflag = true;
                    }
                    self.uint_reg[1] /= divisor as u16;
                    self.uint_reg[1] as i64 / divisor as i64
                }
                6 => {
                    if self.float_reg[0] % divisor != 0.0 {
                        self.rflag = true;
                    }
                    self.float_reg[0] /= divisor;
                    self.float_reg[0] as i64 / divisor as i64
                }
                7 => {
                    if self.float_reg[1] % divisor != 0.0 {
                        self.rflag = true;
                    }
                    self.float_reg[1] /= divisor;
                    self.float_reg[1] as i64 / divisor as i64
                }
                n if n > 5 => {
                    self.report_invalid_register();
                    0
                }
                _ => {
                    if f32::from(self.int_reg[*n as usize]) % divisor != 0.0 {
                        self.rflag = true;
                    }
                    self.int_reg[*n as usize] /= divisor as i16;
                    self.int_reg[*n as usize] as i64 / divisor as i64
                }
            };
            self.check_overflow(new_value, *n as u16);
        }
    }

    pub fn handle_ret(&mut self) {
        /*let temp: i32 = if self.sp >= self.bp {
            (self.sp - 1).into()
        } else {
            (self.sp + 1).into()
        };
        */
        let temp: i32 = self.sp.into();
        if let Some(v) = self.memory[temp as usize] {
            self.pc = v as u16;
            if self.sp > self.bp {
                self.memory[self.sp as usize] = None;
                if self.sp != self.bp {
                    self.sp -= 1;
                }
            } else {
                self.memory[self.sp as usize] = None;
                if self.sp != self.bp {
                    self.sp += 1;
                }
            }
        } else {
            UnrecoverableError::StackUnderflow(self.pc, None).err();

            self.running = false;
            if !CONFIG.debug {
                std::process::exit(1);
            }
        }
    }

    pub fn handle_ld(&mut self, arg1: &Argument, arg2: &Argument) {
        let source = self.get_value(arg2);
        if let Register(n) = arg1 {
            match *n {
                4 => self.uint_reg[0] = source as u16,
                5 => self.uint_reg[1] = source as u16,
                6 => self.float_reg[0] = source,
                7 => self.float_reg[1] = source,
                n if n > 5 => self.report_invalid_register(),
                _ => {
                    self.check_overflow(source as i64, *n as u16);
                    self.int_reg[*n as usize] = source as i16;
                }
            }
        }
    }

    pub fn handle_st(&mut self, arg1: &Argument, arg2: &Argument) {
        let source = self.get_value(arg2);
        if let MemAddr(n) = arg1 {
            self.memory[*n as usize] = Some(source as i16);
        } else if let RegPtr(n) = arg1 {
            let address = self.get_value(&Register(*n));
            self.memory[address as usize] = Some(source as i16);
        }
    }

    pub fn handle_swp(&mut self, arg1: &Argument, arg2: &Argument) {
        let source = self.get_value(arg2);
        if let Register(_) = arg1 {
            let dest = self.get_value(arg1);
            self.set_register_value(arg1, source);
            self.set_register_value(arg2, dest);
        }
    }

    pub fn handle_jz(&mut self, arg: &Argument) {
        if !self.zflag {
            return;
        }
        self.handle_push(&Argument::Literal(self.pc.try_into().unwrap()));
        if let MemAddr(n) = arg {
            self.pc = (*n as u16) - 2;
        } else if let RegPtr(n) = arg {
            self.pc = self.get_value(&Argument::Register(*n)) as u16;
        }
    }

    pub fn handle_cmp(&mut self, arg1: &Argument, arg2: &Argument) {
        let src = self.get_value(arg2);
        if let Register(_) = arg1 {
            let value = self.get_value(arg1);
            let result = value - src;
            self.zflag = (result).abs() < f32::EPSILON;
            self.sflag = result < 0.0;
        }
    }

    pub fn handle_mul(&mut self, arg1: &Argument, arg2: &Argument) {
        let value = self.get_value(arg2);
        if let Register(n) = arg1 {
            let new_value = match *n {
                4 => {
                    self.uint_reg[0] *= value as u16;
                    self.uint_reg[0] as i64 * value as i64
                }
                5 => {
                    self.uint_reg[1] *= value as u16;
                    self.uint_reg[1] as i64 * value as i64
                }
                6 => {
                    self.float_reg[0] *= value;
                    self.float_reg[0] as i64 * value as i64
                }
                7 => {
                    self.float_reg[1] *= value;
                    self.float_reg[1] as i64 * value as i64
                }
                n if n > 5 => {
                    self.report_invalid_register();
                    0
                }
                _ => {
                    self.int_reg[*n as usize] *= value as i16;
                    self.int_reg[*n as usize] as i64 * value as i64
                }
            };
            self.check_overflow(new_value, *n as u16);
        }
    }

    pub fn handle_push(&mut self, arg: &Argument) {
        let mut val: f32 = 0.0;
        if let Literal(l) = arg {
            val = (*l).into();
        }

        if let Register(_) = arg {
            val = self.get_value(arg);
        }
        if self.sp > self.bp || self.backward_stack {
            /*
            for i in self.bp..self.sp {
                if self.memory[i as usize].is_none() {
                    */
            if self.sp != self.bp {
                RecoverableError::BackwardStack(self.pc, None).err();
            } /*
                      self.memory[i as usize] = Some(val as i16);
                      self.sp = i;
                      break;
                  }
              }*/
            loop {
                match self.memory[self.sp as usize] {
                    Some(_) => {
                        self.sp += 1;
                    }
                    None => {
                        self.memory[self.sp as usize] = Some(val as i16);
                        break;
                    }
                }
            }
            if self.sp >= self.bp {
                self.backward_stack = true;
            }
        } else {
            if self.sp == 0 {
                UnrecoverableError::StackOverflow(
                    self.pc,
                    Some("Overflowed while pushing onto stack".to_string()),
                )
                .err();
            }
            loop {
                match self.memory[self.sp as usize] {
                    Some(_) => {
                        self.sp -= 1;
                    }
                    None => {
                        self.memory[self.sp as usize] = Some(val as i16);
                        break;
                    }
                }
            }
            /*
            let mut i = self.bp;
            while i <= self.bp {
                if self.memory[i as usize].is_none() {
                    self.memory[i as usize] = Some(val as i16);
                    self.sp = i; // set the stack pointer
                    break;
                }
                i -= 1;
            }
            */
        }
    }
    pub fn handle_mov(&mut self, arg1: &Argument, arg2: &Argument) {
        let value = self.get_value(arg2);
        if let Register(n) = arg1 {
            match *n {
                4 => self.uint_reg[0] = value as u16,
                5 => self.uint_reg[1] = value as u16,
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
            0_u16..=3_u16 => {
                println!("{}", self.int_reg[code as usize]);
            }
            4 => println!("{}", self.uint_reg[0]),
            5 => println!("{}", self.uint_reg[1]),
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
                        self.handle_segmentation_fault(
                            "Segmentation fault. Memory index out of bounds on interrupt call 8.",
                        );
                        return;
                    }
                }
            }
            9 => {
                let stdin = 0;
                let termios = Termios::from_fd(stdin).unwrap();
                let mut new_termios = termios;
                new_termios.c_lflag &= !(ICANON | ECHO);
                tcsetattr(stdin, TCSANOW, &new_termios).unwrap();
                let stdout = io::stdout();
                let mut buffer = [0; 1]; // thank you random stranger on stack overflow
                stdout.lock().flush().unwrap(); // for graciously providing me with this
                io::stdin().read_exact(&mut buffer).unwrap(); // wonderful code block to read
                self.int_reg[0] = buffer[0] as i16; // a single letter/character from stdin
                tcsetattr(stdin, TCSANOW, &termios).unwrap();
            }
            10 => std::thread::sleep(std::time::Duration::from_secs(1)),
            11 => self.zflag = true,
            12 => self.zflag = false,
            13 => self.zflag = !self.zflag,
            20 => {
                self.max_clk = Some(self.int_reg[0] as usize);
            }
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

            60 => self.sp = self.uint_reg[0],
            61 => self.bp = self.uint_reg[0],

            // 10 - 20 set flags
            // 20 - 30 unset them
            // 30 - 40 invert them
            _ => RecoverableError::UnknownFlag(
                self.pc,
                Some(String::from("Occurred whilst handling INT")),
            )
            .err(),
        }
    }
}
