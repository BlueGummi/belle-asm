use crate::{RecoverableError, UnrecoverableError, CONFIG, CPU};

impl CPU {
    pub fn report_invalid_register(&mut self) {
        UnrecoverableError::InvalidRegister(
            self.pc,
            Some("The register number is too large.".to_string()),
        )
        .err();
        if CONFIG.debug {
            self.running = false;
            return;
        }
        if !CONFIG.quiet {
            println!("Attempting to recover by restarting...");
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        self.pc = self.starts_at;
    }

    pub fn report_unknown_flag(&self, instruction: &str) {
        RecoverableError::UnknownFlag(
            self.pc,
            Some(format!("Unknown flag in {instruction} instruction")),
        )
        .err();
    }

    pub fn report_divide_by_zero(&mut self) {
        UnrecoverableError::DivideByZero(self.pc, Some("Attempted to divide by zero.".to_string()))
            .err();
        if CONFIG.debug {
            self.running = false;
            return;
        }
        if !CONFIG.quiet {
            println!("Attempting to recover by restarting...");
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        self.pc = self.starts_at;
    }
    pub fn check_overflow(&mut self, new_value: i64, register: u16) {
        let overflowed = match register {
            0..=3 => new_value > i64::from(i16::MAX) || new_value < i64::from(i16::MIN),
            4..=5 => new_value > i64::from(u16::MAX) || new_value < i64::from(u16::MIN),
            6..=7 => new_value > f32::MAX as i64 || new_value < f32::MIN as i64,
            _ => true,
        };
        if overflowed {
            RecoverableError::Overflow(self.pc, Some("Overflowed a register.".to_string())).err();
            self.oflag = true;
            if self.hlt_on_overflow {
                self.running = false;
                if CONFIG.verbose {
                    println!("Halting...");
                }
                if CONFIG.pretty {
                    for i in 0..=3 {
                        println!(
                            "Register {}: {}, {:016b}, {:04x}",
                            i, self.int_reg[i], self.int_reg[i], self.int_reg[i]
                        );
                    }
                    for i in 0..=1 {
                        println!("Uint Register {}: {}", i, self.uint_reg[i]);
                    }
                    for i in 0..=1 {
                        println!("Float Register {}: {}", i, self.float_reg[i]);
                    }
                }
                if !CONFIG.debug {} // dumb hack for a dumb bug (cpu would overflow twice)
            }
        }
    }

    pub fn handle_segmentation_fault(&mut self, message: &str) {
        UnrecoverableError::SegmentationFault(self.pc, Some(message.to_string())).err();
        self.running = false;
        return;
    }
}
